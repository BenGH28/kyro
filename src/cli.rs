use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kyro", about = "Read the Bible on the commandline")]
pub enum Command {
    /// search for a passage and print to console
    Search { passage: String },
    /// start reading a passage of scripture in a buffer
    Read { passage: String },
    /// display the verse of the day
    Today,
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Search { passage } => println!("Searching the Bible for {}", passage),
            Command::Read { passage } => println!("Reading {}", passage),
            Command::Today => println!("Todays vers is John 3:16"),
        }
    }
}
