use anyhow::Context as _;

use crate::storage;
use crate::Config;

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

    let file_path = storage::get_path_to_bible_file(config)?
        .into_string()
        .unwrap();

    let bible_string = storage::read_bible_from_file(&file_path)?;

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

    let file_path = storage::get_path_to_bible_file(config)?
        .into_string()
        .unwrap();
    let bible_string = storage::read_bible_from_file(&file_path)?;

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

#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::Config;

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
    }

    #[test]
    #[ignore]
    fn test_get_passage_correct() -> anyhow::Result<()> {
        let vs1 = "In the beginning God created the heavens and the earth.";

        storage::download_bible(&Config::default())?;
        let actual = get_passage(&Config::default(), "Gen", "1:1").unwrap();

        assert_eq!(vs1, actual);
        Ok(())
    }

    #[test]
    #[ignore]
    fn passage_range_test() -> anyhow::Result<()> {
        let vs1 = "In the beginning God created the heavens and the earth.";
        let vs2 = "And the earth was waste and void; and darkness was upon the face of the deep: and the Spirit of God moved upon the face of the waters";

        storage::download_bible(&Config::default())?;

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
