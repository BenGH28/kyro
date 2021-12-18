#![allow(unused_variables)]
#![allow(dead_code)]
mod config;

use std::fs;

use anyhow::Context as _;
pub use config::Config;
use directories_next::ProjectDirs;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

///Get the raw github URL from the config
pub fn get_bible_url(lang_code: &str, version: &str) -> String {
    let url: String = format!(
        r#"https://raw.githubusercontent.com/gratis-bible/bible/master/{}/{}.xml"#,
        lang_code, version
    );
    url
}

///Get the directory to store all the Bibles
pub fn get_data_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", "BenGH28", "kyro") {
        let data_dir: &Path = proj_dirs.data_dir();
        return Some(data_dir.to_owned());
    }
    None
}

///Download the Bible and save it on your computer
pub fn download_bible(lang_code: &str, config: &Config) -> anyhow::Result<()> {
    let url = get_bible_url(lang_code, &config.version);
    save_to_pc(&url, config)?;
    Ok(())
}

///Save a Bible in xml to a file on the computer under the XDG format (ie. $HOME/.local/share/kyro/)
pub fn save_to_pc(url: &str, config: &Config) -> anyhow::Result<()> {
    //get data_dir: $HOME/.local/share/kyro/
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;

    //get the path to the language sub directory (ie. $HOME/.local/share/kyro/english/)
    let language_dir: PathBuf = data_dir.join(&config.language);

    //the full path to the file (ie. $HOME/.local/share/kyro/english/asv.xml)
    let file_path: PathBuf = language_dir.join(config.get_file_name());

    //if the directory doesn't exist then lets create it
    if !language_dir.is_dir() {
        std::fs::create_dir_all(language_dir)
            .context("could not create kyro project data directory")?;
    }

    //if the file doesn't exist then we can write it
    if !file_path.is_file() {
        //create and write the Bible to a file
        let bible: String = get_bible_text(url)?;
        let mut file: File = File::create(file_path).context("could not write Bible to file")?;
        file.write_all(bible.as_bytes())?;
    }
    Ok(())
}

fn read_bible_from_file(path: &str) -> anyhow::Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

///parse the xml bible into a Document
fn parse(bible_string: &str) -> anyhow::Result<roxmltree::Document> {
    let bible = roxmltree::Document::parse(bible_string)?;
    Ok(bible)
}

fn get_verse_from_xml(
    parsed_bible: &roxmltree::Document,
    search_string: &str,
) -> anyhow::Result<String> {
    //this only gets a single verse
    let xml_vs: roxmltree::Node = parsed_bible
        .descendants()
        .find(|n| n.attribute("osisID") == Some(search_string))
        .context("could not find selected passage")?;
    let text = xml_vs.first_child().unwrap().text().unwrap().to_owned();
    Ok(text)
}
fn get_single_verse(
    config: &Config,
    book: &str,
    chapter: &str,
    verse: &str,
) -> anyhow::Result<String> {
    let xml_format_ch_vs: String = format!("{}.{}.{}", book, chapter, verse);

    let file_path = get_path_to_bible_file(config)?.into_string().unwrap();

    let bible_string = read_bible_from_file(&file_path)?;

    //the entire bible is at my disposal now
    let parsed_bible = parse(&bible_string)?;
    let text = get_verse_from_xml(&parsed_bible, &xml_format_ch_vs)?;
    Ok(text)
}

fn get_range_verse(
    config: &Config,
    book: &str,
    chapter: &str,
    verses: &str,
) -> anyhow::Result<String> {
    let mut passage: String = "".to_string();
    let range: Vec<&str> = verses.split('-').collect();

    let begin: i32 = range[0]
        .parse()
        .context("beginning of verse range is not a valid number")?;

    let end: i32 = range[1]
        .parse()
        .context("end of verse range is not a valid number")?;

    let file_path = get_path_to_bible_file(config)?.into_string().unwrap();
    let bible_string = read_bible_from_file(&file_path)?;

    //the entire bible is at my disposal now
    let parsed_bible = parse(&bible_string)?;

    for i in begin..end + 1 {
        let xml_format_ch_vs = format!("{}.{}.{}", book, chapter, i);
        let verse = get_verse_from_xml(&parsed_bible, &xml_format_ch_vs)?;
        passage += &verse;
    }
    Ok(passage)
}
pub fn get_passage(config: &Config, book: &str, chapter_verse: &str) -> anyhow::Result<String> {
    //separate the chapter from the verse(s)
    let chpt_vs_split: Vec<&str> = chapter_verse.split(':').collect();
    let chapter: &str = chpt_vs_split[0];
    let verses: &str = chpt_vs_split[1];
    let passage: String;

    if verses.contains('-') {
        passage = get_range_verse(config, book, chapter, verses)?;
    } else {
        passage = get_single_verse(config, book, chapter, verses)?;
    }
    Ok(passage)
}

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

///get the absolute path to the bible xml file
pub fn get_path_to_bible_file(config: &Config) -> anyhow::Result<OsString> {
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
    let file_path: OsString = data_dir
        .join(&config.language)
        .join(config.get_file_name())
        .into_os_string();
    Ok(file_path)
}

///Get the text from the gratis-bible github account
pub fn get_bible_text(url: &str) -> anyhow::Result<String> {
    let bible: String = reqwest::blocking::get(url)?.text()?;
    Ok(bible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let doc = parse(
            r#"
        <verse osisID='Gen.1.1'>In the beginning God created the heavens and the earth.</verse>
              "#,
        )
        .unwrap();

        let elem: roxmltree::Node = doc
            .descendants()
            .find(|n| n.attribute("osisID") == Some("Gen.1.1"))
            .unwrap();
        assert!(elem.has_tag_name("verse"));

        let text = doc.root_element().first_child().unwrap().text().unwrap();
        assert_eq!(
            text,
            "In the beginning God created the heavens and the earth."
        );
    }
    #[test]
    fn test_get_bible_text() {
        let config = Config::default();
        let code = config.get_language_code().unwrap();
        let url: String = get_bible_url(&code, &config.version);
        let text: String = get_bible_text(&url).unwrap();
        assert_ne!("", text);
    }
    #[test]
    fn test_get_passage_correct() -> anyhow::Result<()> {
        let vs1 = "In the beginning God created the heavens and the earth.";

        let config = Config::get_config()?;
        let lang_code = config.get_language_code().context("Unknown language")?;
        download_bible(&lang_code, &config)?;
        let actual = get_passage(&Config::default(), "Gen", "1:1").unwrap();

        assert_eq!(vs1, actual);
        Ok(())
    }

    #[test]
    fn passage_range_test() -> anyhow::Result<()> {
        let vs1 = "In the beginning God created the heavens and the earth.";
        let vs2 = "And the earth was waste and void; and darkness was upon the face of the deep: and the Spirit of God moved upon the face of the waters";

        let config = Config::get_config()?;
        let lang_code = config.get_language_code().context("Unknown language")?;
        download_bible(&lang_code, &config)?;

        let actual = get_passage(&Config::default(), "Gen", "1:1-2").unwrap();
        let expected = vs1.to_owned() + &vs2.to_owned();

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_get_passage_err() {
        assert!(get_passage(&Config::default(), "gen", "1:2").is_err());
    }
}
