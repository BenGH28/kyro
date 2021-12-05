use kyro::{print_passage, read_passage, today, Config};
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
                book,
                chapter_verse,
            } => print_passage(config, book, chapter_verse),
            Command::Read {
                book,
                chapter_verse,
            } => read_passage(book, chapter_verse),
            Command::Today => today(),
        }
    }
}
