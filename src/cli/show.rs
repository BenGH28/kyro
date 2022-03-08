use std::io::{self, Write};

use anyhow::Context;
use chrono::{Datelike, Local};
use pager::Pager;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    bible::{
        book::{Book, BOOK_ORDER},
        chapter::Chapter,
        paragraph::Paragraph,
        verse::Verse,
    },
    bible_as_str, get_path_to_bible_file, Config, Query,
};

pub fn find_pgh_idx(ch: &Chapter, verse_num: u32) -> Option<usize> {
    for i in 0..ch.paragraphs.len() {
        let opt = ch.paragraphs[i]
            .verses
            .iter()
            .find(|v| v.number == verse_num);
        if opt.is_some() {
            return Some(i);
        }
    }
    None
}

//HACK: the nesting is so gross...
pub fn print_first_chapter(ch: &Chapter, query: &Query) -> anyhow::Result<()> {
    let start_vs = query.entry_point.verse;
    let end_vs = query.end_point.verse;

    //the paragraph that contains the starting verse
    let first_pgh_idx =
        find_pgh_idx(ch, start_vs).context(format!("cannot find verse {} ", start_vs))?;
    // option for the last paragraph of the query - only Some if the query is within a single
    // chapter
    let last_pgh_idx_opt = find_pgh_idx(ch, end_vs);

    if query.is_range() {
        let last_idx: usize;
        //query doesn't span outside of the chapter
        if query.is_internal_range() {
            if let Some(ending) = last_pgh_idx_opt {
                last_idx = ending;

                if last_idx == first_pgh_idx {
                    println!("{}", ch.paragraphs[first_pgh_idx]);
                } else {
                    // be carefull both idx's aren't the same
                    for i in first_pgh_idx..last_idx {
                        println!("{}", ch.paragraphs[i]);
                    }
                }
            }
        } else {
            //range spans multiple chapters
            last_idx = ch.paragraphs.len();
            for i in first_pgh_idx..last_idx {
                println!("{}", ch.paragraphs[i]);
            }
        }
    } else {
        //single verse queried
        let pgh: &Paragraph = &ch.paragraphs[first_pgh_idx];
        let vs: &Verse = pgh
            .verses
            .iter()
            .find(|v| v.number == start_vs)
            .expect("cannot find the desired verse");
        println!("{}", vs);
    }
    Ok(())
}

pub fn print_last_chapter(ch: &Chapter, query: &Query) {
    //for the last chapter print from the beginning to the ending point
    //filter the paragraphs so that we only get the ones upto and including the ending vs
    let end_vs = query.end_point.verse;
    let final_phgs_iter = ch.paragraphs.iter().filter(|p| {
        let opt = p.verses.iter().find(|v| v.number <= end_vs);
        opt.is_some()
    });
    for pgh in final_phgs_iter {
        println!("{}", pgh);
    }
}

pub fn print_passage(book: &mut Book, query: &mut Query) -> anyhow::Result<()> {
    //get the start and end points
    let start_chpt = query.entry_point.chpt;
    let end_chpt = query.end_point.chpt;

    let chapters_iter = book.chapters.iter().filter(|c| {
        if end_chpt != 0 {
            c.number >= start_chpt && c.number <= end_chpt
        } else {
            c.number == start_chpt
        }
    });

    for ch in chapters_iter {
        if ch.number == start_chpt {
            print_first_chapter(ch, query)?;
        } else if ch.number < end_chpt {
            //for all chapters in-between just print them to the screen
            println!("{}", ch);
        } else {
            print_last_chapter(ch, query);
        }
    }

    Ok(())
}

pub fn paginate(book: &Book, less_cmd: &str) -> anyhow::Result<()> {
    Pager::with_pager(less_cmd).setup();

    let out: Vec<String> = book.chapters.iter().map(ToString::to_string).collect();
    let result = io::stdout().write_all(out.join("\n").as_bytes());

    //HACK: pipes can break when the user quits the pager before reading the entire Book so
    //don't panic please
    if result.is_ok() {
        return Ok(());
    }
    Ok(())
}

pub fn read_passage(book: &Book, query_opt: Option<&Query>) -> anyhow::Result<()> {
    match query_opt {
        Some(query) => {
            let less_cmd = format!("less -p ^CHAPTER_{}$", query.entry_point.chpt);
            Ok(paginate(book, &less_cmd)?)
        }
        None => {
            let less_cmd = "less";
            Ok(paginate(book, less_cmd)?)
        }
    }
}

pub fn num_gen(rng: &mut ChaCha8Rng, end_of_range: usize) -> usize {
    rng.gen_range(1..end_of_range)
}
pub fn today(config: &Config) -> anyhow::Result<()> {
    let now = Local::now();
    let naive = now.naive_local();
    let year = naive.year() as u32;
    let month = naive.month();
    let day = naive.day();

    let seed: u64 = (year + month + day).into();
    let num_books = 66_usize;

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let book_num = num_gen(&mut rng, num_books) as u32;

    let book_title: String = BOOK_ORDER
        .get(&book_num)
        .expect("book selected doesn't exist")
        .to_string();
    let book = setup_a_book(book_title, config)?;

    let chpt_num = num_gen(&mut rng, book.chapters.len());
    let chpt: &Chapter = &book.chapters[chpt_num];

    let pgh_num = num_gen(&mut rng, chpt.paragraphs.len());
    let pgh: &Paragraph = &chpt.paragraphs[pgh_num];

    let vs_num = num_gen(&mut rng, pgh.verses.len());
    let vs: &Verse = &pgh.verses[vs_num];

    println!("{} {}:{}\n{}", book.title, chpt.number, vs.number, vs);

    Ok(())
}

pub fn setup_a_book(book_title: String, config: &Config) -> anyhow::Result<Book> {
    let bible_str = bible_as_str(get_path_to_bible_file(config)?)?;
    let bible_doc = roxmltree::Document::parse(&bible_str)?;
    let title = book_title;
    let book: Book = Book::new(title, &bible_doc)?;
    Ok(book)
}
