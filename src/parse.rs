use anyhow::Context as _;

use crate::Config;
use crate::{bible_as_str, get_path_to_bible_file};

///parse the xml bible into a Document
pub fn parse(bible_string: &str) -> anyhow::Result<roxmltree::Document> {
    let bible = roxmltree::Document::parse(bible_string)?;
    Ok(bible)
}


fn get_range_verse(
    config: &Config,
    book: &str,
    chapter: &str,
    verses: &str,
) -> anyhow::Result<String> {
    let passage: String = "".to_string();
    let range: Vec<&str> = verses.split('-').collect();

    let begin: i32 = range[0]
        .parse()
        .context("beginning of verse range is not a valid number")?;

    let end: i32 = range[1]
        .parse()
        .context("end of verse range is not a valid number")?;

    let file_path = get_path_to_bible_file(config)?.into_string().unwrap();
    let bible_string = bible_as_str(&file_path)?;

    //the entire bible is at my disposal now
    let parsed_bible = parse(&bible_string)?;

    for i in begin..end + 1 {
        let xml_format_ch_vs = format!("{}.{}.{}", book, chapter, i);
    }

    todo!()
}
pub fn get_passage(config: &Config, book: &str, chapter_verse: &str) -> anyhow::Result<String> {
    //separate the chapter from the verse(s)
    let chpt_vs_split: Vec<&str> = chapter_verse.split(':').collect();
    let chapter: &str = chpt_vs_split[0];
    let verses: &str = chpt_vs_split[1];
    let _passage: String;

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::download_bible;
    use crate::parse::*;
    use crate::Config;

    #[test]
    #[ignore]
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
    }

    #[test]
    #[ignore]
    fn test_get_passage_correct() -> anyhow::Result<()> {
        let vs1 = "In the beginning God created the heavens and the earth.";

        download_bible(&Config::default())?;
        let actual = get_passage(&Config::default(), "Gen", "1:1").unwrap();

        assert_eq!(vs1, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn passage_range_test() -> anyhow::Result<()> {
        let vs1 = "In the beginning God created the heavens and the earth.";
        let vs2 = "And the earth was waste and void; and darkness was upon the face of the deep: and the Spirit of God moved upon the face of the waters";

        download_bible(&Config::default())?;

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
