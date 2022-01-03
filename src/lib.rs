mod bible;
mod cli;
mod config;
mod parse;
mod storage;

use crate::parse::get_passage;
pub use crate::storage::*;
pub use cli::Command;
pub use config::Config;

///print the passage of scripture to the terminal
pub fn print_passage(config: &Config, book: &str, chapter_verse: &str) -> anyhow::Result<()> {
    let text = get_passage(config, book, chapter_verse)?;
    println!("{}", text);
    Ok(())
}

pub fn read_passage(book: &str, chapter_verse: &str) -> anyhow::Result<()> {
    todo!()
}

pub fn today() -> anyhow::Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bible_text() {
        let url: String = storage::get_bible_url(&Config::default());
        let text: bytes::Bytes = storage::get_bible_zip(&url).unwrap();
        assert_ne!("", text);
    }
}
