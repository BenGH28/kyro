pub mod query;
pub mod show;

use crate::Config;
use crate::Query;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kyro", about = "Read the Bible on the commandline")]
pub enum Command {
    /// Search for a passage and print to console
    Search { book: String, chapter_verse: String },
    /// Start reading the bible with `less`
    Read {
        book: String,
        chapter: Option<String>,
    },
    /// Display the verse of the day
    Today,
}

impl Command {
    pub fn run(&self, config: &Config) -> anyhow::Result<()> {
        match self {
            Command::Search {
                book: book_title,
                chapter_verse,
            } => {
                let mut book = show::setup_a_book(book_title.to_string(), config)?;
                let mut query = Query::setup_query(chapter_verse.to_string())?;
                show::print_passage(&mut book, &mut query)
            }
            Command::Read {
                book: book_title,
                chapter: chapter_verse,
            } => {
                let book = show::setup_a_book(book_title.to_string(), config)?;
                if let Some(passage) = chapter_verse {
                    let query = Query::setup_query(passage.to_string())?;
                    show::read_passage(&book, Some(&query))
                } else {
                    show::read_passage(&book, None)
                }
            }
            Command::Today => show::today(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_run() {
        let cmd = Command::Search {
            book: "John".to_string(),
            chapter_verse: "3:16".to_string(),
        };
        cmd.run(&Config::default()).unwrap();
    }
}
