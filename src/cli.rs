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
    pub fn run(&self) {
        match self {
            Command::Search {
                book,
                chapter_verse,
            } => println!("Searching the Bible for {} {}", book, chapter_verse),
            Command::Read {
                book,
                chapter_verse,
            } => println!("Reading {} {}", book, chapter_verse),
            Command::Today => println!("Todays verse is John 3:16"),
        }
    }
}
