use crate::bible::book::Book;
use crate::bible::chapter::Chapter;
use crate::bible::passage::Point;
use crate::{bible_as_str, get_path_to_bible_file, Config};
use anyhow::Context;
use structopt::StructOpt;

pub struct Query {
    pub entry_point: Point,
    pub end_point: Point,
    pub vs_next: u32,
    pub chpt_next: u32,
}

impl Query {
    pub fn new(entry_point: Point, end_point: Point) -> Self {
        Self {
            entry_point,
            end_point,
            vs_next: 0,
            chpt_next: 0,
        }
    }

    pub fn next_chpt(&mut self) {
        if self.chpt_next == 0 {
            self.chpt_next = self.entry_point.chpt + 1;
        } else {
            self.chpt_next += 1;
        }
    }

    pub fn next_vs(&mut self) {
        if self.vs_next == 0 {
            self.vs_next = self.entry_point.verse + 1;
        } else {
            self.vs_next += 1;
        }
    }
}

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
                let mut query = setup_query(chapter_verse.to_string())?;
                Command::print_passage(&mut book, &mut query)
            }
            Command::Read {
                book: book_title,
                chapter_verse,
            } => {
                let book = setup_a_book(book_title.to_string(), config)?;
                let query = setup_query(chapter_verse.to_string())?;
                Command::read_passage(book)
            }
            Command::Today => Command::today(),
        }
    }

    fn print_passage(book: &mut Book, query: &mut Query) -> anyhow::Result<()> {
        //get the start and end points
        let start_chpt = query.entry_point.chpt;
        let end_chpt = query.end_point.chpt;

        let start_vs = query.entry_point.verse;
        let end_vs = query.end_point.verse;

        //PROBLEM: why wont this work?
        //use an iterator over the chapters
        let iter = book.chapters.iter().take_while(|c| c.number == start_chpt);
        for c in iter {
            println!("{}", c);
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

fn setup_query(chapter_verse: String) -> anyhow::Result<Query> {
    let (start_point, end_point) = split_cli_query(&chapter_verse)?;
    let query = Query::new(start_point, end_point);
    Ok(query)
}

fn get_query_start(start_vec: &[&str]) -> anyhow::Result<Point> {
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

    Ok(start)
}
pub fn split_cli_query(chpt_vs_query: &str) -> anyhow::Result<(Point, Point)> {
    let dash = '-';
    let colon = ':';

    // if the user just enters the book title and no chapter:verse then start them off at the
    // beginning of the book
    if chpt_vs_query.is_empty() {
        return Ok((Point::new(1, 1), Point::new(0, 0)));
    }

    let split: Vec<&str> = chpt_vs_query.split(dash).collect();

    let start_vec: Vec<&str> = split[0].split(colon).collect();
    let start = get_query_start(&start_vec)?;

    let end: Point;
    if split.len() == 1 {
        end = Point::new(0, 0);
    } else {
        //WARNING:this only exists if there is a dash
        let end_vec: Vec<&str> = split[1].split(colon).collect();

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

        end = Point {
            chpt: end_chpt,
            verse: end_vs,
        };
    }

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn query_fixture() -> Query {
        let start = Point::new(9, 3);
        let end = Point::new(10, 8);
        Query::new(start, end)
    }

    #[rstest]
    fn query_next_chpt(mut query_fixture: Query) {
        query_fixture.next_chpt();
        let expected = 10;
        let actual = query_fixture.chpt_next;
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn query_next_vs(mut query_fixture: Query) {
        query_fixture.next_vs();
        let expected = 4;
        let actual = query_fixture.vs_next;
        assert_eq!(expected, actual);
    }
}
