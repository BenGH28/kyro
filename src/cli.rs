use crate::{
    bible::{
        book::Book,
        paragraph::Paragraph,
        passage::{Navigate, Point},
    },
    bible_as_str, get_path_to_bible_file, Config,
};
use anyhow::Context;
use structopt::StructOpt;

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
                let book = setup_a_book(book_title.to_string(), chapter_verse.to_string(), config)?;
                Command::print_passage(book)
            }
            Command::Read {
                book: book_title,
                chapter_verse,
            } => {
                let book = setup_a_book(book_title.to_string(), chapter_verse.to_string(), config)?;
                Command::read_passage(book)
            }
            Command::Today => Command::today(),
        }
    }

    fn print_passage(book: Book) -> anyhow::Result<()> {
        //get the chapter(s) that contain the passage
        let chpt_begin = book.entry_point.chpt;
        let verse_begin = book.entry_point.verse;
        let chpt_end = book.end_point.chpt;
        let verse_end = book.end_point.verse;

        let mut result: Vec<&Paragraph> = Vec::new();

        let chpt = book.begin()?;
        let first_paragraph = chpt
            .paragraphs
            .iter()
            .find(|p| {
                let result = p.verses.iter().find(|v| v.number == verse_begin);
                match result {
                    Some(v) => v.number == verse_begin,
                    None => false,
                }
            })
            .context(format!(
                "couldn't find verse {} in chapter {}",
                verse_begin, chpt_begin
            ))?;

        result.push(first_paragraph);
        //while let Some(p) = chpt.next() {
        //    result.push(p);
        //}
        ////get the verse(s) from those chapters and display them

        Ok(())
    }

    fn read_passage(book: Book) -> anyhow::Result<()> {
        todo!()
    }

    fn today() -> anyhow::Result<()> {
        todo!()
    }
}

fn setup_a_book(
    book_title: String,
    chapter_verse: String,
    config: &Config,
) -> anyhow::Result<Book> {
    let bible_str = bible_as_str(get_path_to_bible_file(config)?)?;
    let bible_doc = roxmltree::Document::parse(&bible_str)?;
    let title = book_title;
    let (start_point, end_point) = split_cli_query(&chapter_verse)?;
    let book: Book = Book::new(title, start_point, end_point, &bible_doc)?;
    Ok(book)
}
fn split_cli_query(chpt_vs_query: &str) -> anyhow::Result<(Point, Point)> {
    let dash = '-';
    let colon = ':';

    // if the user just enters the book title and no chapter:verse then start them off at the
    // beginning of the book
    if chpt_vs_query.is_empty() {
        return Ok((Point::new(1, 1), Point::new(0, 0)));
    }

    let split: Vec<&str> = chpt_vs_query.split(dash).collect();
    let start_vec: Vec<&str> = split[0].split(colon).collect();
    let end_vec: Vec<&str> = split[1].split(colon).collect();

    let start_chpt = start_vec[0]
        .parse::<u32>()
        .context("starting chapter invalid")?;

    let start_vs = start_vec[1]
        .parse::<u32>()
        .context("starting verse is invalid")?;

    let start = Point {
        chpt: start_chpt,
        verse: start_vs,
    };

    let has_chpt = end_vec.len() == 2;
    let end_chpt = if has_chpt {
        end_vec[0]
            .parse::<u32>()
            .context("ending chapter is invalid")?
    } else {
        0
    };

    let end_vs = if has_chpt {
        end_vec[1]
            .parse::<u32>()
            .context("ending verse is invalid")?
    } else {
        end_vec[0]
            .parse::<u32>()
            .context("ending verse is invalid")?
    };

    let end = Point {
        chpt: end_chpt,
        verse: end_vs,
    };

    Ok((start, end))
}
