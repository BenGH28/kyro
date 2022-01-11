use crate::{
    bible::{book::Book, passage::Point},
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
                print_passage(config, book)
            }
            Command::Read {
                book: book_title,
                chapter_verse,
            } => {
                let book = setup_a_book(book_title.to_string(), chapter_verse.to_string(), config)?;
                read_passage(book)
            }
            Command::Today => today(),
        }
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

    let split: Vec<&str> = chpt_vs_query.split(dash).collect();
    let start_point: Vec<&str> = split[0].split(colon).collect();
    let end_point: Vec<&str> = split[1].split(colon).collect();

    let start_chpt = start_point[0]
        .parse::<u32>()
        .context("starting chapter invalid")?;

    let start_vs = start_point[1]
        .parse::<u32>()
        .context("starting verse is invalid")?;

    let start = Point {
        chpt: start_chpt,
        verse: start_vs,
    };

    let has_chpt = end_point.len() == 1;
    let end_chpt = if has_chpt {
        0
    } else {
        end_point[0]
            .parse::<u32>()
            .context("ending chapter is invalid")?
    };

    let end_vs = if has_chpt {
        end_point[0]
            .parse::<u32>()
            .context("ending verse is invalid")?
    } else {
        0
    };

    let end = Point {
        chpt: end_chpt,
        verse: end_vs,
    };

    Ok((start, end))
}

fn print_passage(config: &Config, book: Book) -> anyhow::Result<()> {
    todo!()
}

fn read_passage(book: Book) -> anyhow::Result<()> {
    todo!()
}

fn today() -> anyhow::Result<()> {
    todo!()
}
