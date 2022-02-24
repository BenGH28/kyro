use crate::bible::{book::Book, chapter::Chapter, passage::Point};
use crate::Query;
use crate::{bible_as_str, get_path_to_bible_file, Config};
use anyhow::Context;
use structopt::StructOpt;

pub mod query;
#[derive(StructOpt)]
#[structopt(name = "kyro", about = "Read the Bible on the commandline")]
pub enum Command {
    /// search for a passage and print to console
    Search { book: String, chapter_verse: String },
    /// start reading a passage of scripture in a buffer
    Read { book: String, chapter_verse: String },
    /// display the verse of the day
    Today,
}

impl Command {
    pub fn run(&self, config: &Config) -> anyhow::Result<()> {
        match self {
            Command::Search {
                book: book_title,
                chapter_verse,
            } => {
                let mut book = setup_a_book(book_title.to_string(), config)?;
                let mut query = Query::setup_query(chapter_verse.to_string())?;
                Command::print_passage(&mut book, &mut query)
            }
            Command::Read {
                book: book_title,
                chapter_verse,
            } => {
                let book = setup_a_book(book_title.to_string(), config)?;
                let query = Query::setup_query(chapter_verse.to_string())?;
                Command::read_passage(book)
            }
            Command::Today => Command::today(),
        }
    }

    fn find_pgh_idx(ch: &Chapter, verse_num: u32) -> Option<usize> {
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
    fn print_first_chapter(ch: &Chapter, query: &Query) -> anyhow::Result<()> {
        let start_vs = query.entry_point.verse;
        let end_vs = query.end_point.verse;

        let first_pgh_idx = Command::find_pgh_idx(ch, start_vs)
            .context(format!("cannot find verse {} ", start_vs))?;
        let last_pgh_idx_opt = Command::find_pgh_idx(ch, end_vs);

        if query.is_range() {
            let last_idx: usize;
            //query doesn't span outside of the chapter
            if query.is_internal_range() {
                dbg!("internal");
                if let Some(ending) = last_pgh_idx_opt {
                    last_idx = ending;
                    for i in first_pgh_idx..last_idx {
                        println!("{}", ch.paragraphs[i]);
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
            println!("{}", ch.paragraphs[first_pgh_idx]);
        }
        Ok(())
    }

    fn print_last_chapter(ch: &Chapter, query: &Query) {
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

    fn print_passage(book: &mut Book, query: &mut Query) -> anyhow::Result<()> {
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
                Command::print_first_chapter(ch, query)?;
            } else if ch.number < end_chpt {
                //for all chapters in-between just print them to the screen
                println!("{}", ch);
            } else {
                Command::print_last_chapter(ch, query);
            }
        }

        Ok(())
    }

    fn read_passage(book: Book) -> anyhow::Result<()> {
        todo!()
    }

    fn today() -> anyhow::Result<()> {
        todo!()
    }
}

fn setup_a_book(book_title: String, config: &Config) -> anyhow::Result<Book> {
    let bible_str = bible_as_str(get_path_to_bible_file(config)?)?;
    let bible_doc = roxmltree::Document::parse(&bible_str)?;
    let title = book_title;
    let book: Book = Book::new(title, &bible_doc)?;
    Ok(book)
}
