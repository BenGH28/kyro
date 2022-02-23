use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

use anyhow::Context;
use once_cell::sync::Lazy;
use roxmltree::Node;

use super::chapter::Chapter;

use super::paragraph::Paragraph;
use super::verse::Verse;

const CHPT_NUM_TAG: &str = "c";
const ALT_CHPT_NUM_TAG_1: &str = "ca";
const ALT_CHPT_NUM_TAG_2: &str = "cp";
const BOOK_TAG: &str = "book";
const P_PARA_TAG: &str = "p";
const Q_PARA_TAG: &str = "q";
const WORD_TAG: &str = "w";
const ID_TAG: &str = "id";
const NEW_LN: &str = "\n";
const VERSE_TAG: &str = "v";

static BOOK_TITLE_ID: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Genesis".to_string(), "GEN".to_string());
    map.insert("Exodus".to_string(), "EXO".to_string());
    map.insert("Leviticus".to_string(), "LEV".to_string());
    map.insert("Numbers".to_string(), "NUM".to_string());
    map.insert("Deuteronomy".to_string(), "DEU".to_string());
    map.insert("Joshua".to_string(), "JOS".to_string());
    map.insert("Judges".to_string(), "JDG".to_string());
    map.insert("Ruth".to_string(), "RUT".to_string());
    map.insert("1 Samuel".to_string(), "1SA".to_string());
    map.insert("2 Samuel".to_string(), "2SA".to_string());
    map.insert("1 Kings".to_string(), "1KI".to_string());
    map.insert("2 Kings".to_string(), "2KI".to_string());
    map.insert("1 Chronicles".to_string(), "1CH".to_string());
    map.insert("2 Chronicles".to_string(), "2CH".to_string());
    map.insert("Ezra".to_string(), "EZR".to_string());
    map.insert("Nehemiah".to_string(), "NEH".to_string());
    map.insert("Esther".to_string(), "EST".to_string());
    map.insert("Job".to_string(), "JOB".to_string());
    map.insert("Psalms".to_string(), "PSA".to_string());
    map.insert("Proverbs".to_string(), "PRO".to_string());
    map.insert("Ecclesiastes".to_string(), "ECC".to_string());
    map.insert("Song of Solomon".to_string(), "SNG".to_string());
    map.insert("Isaiah".to_string(), "ISA".to_string());
    map.insert("Jeremiah".to_string(), "JER".to_string());
    map.insert("Lamentations".to_string(), "LAM".to_string());
    map.insert("Ezekiel".to_string(), "EZK".to_string());
    map.insert("Daniel".to_string(), "DAN".to_string());
    map.insert("Hosea".to_string(), "HOS".to_string());
    map.insert("Joel".to_string(), "JOL".to_string());
    map.insert("Amos".to_string(), "AMO".to_string());
    map.insert("Obadiah".to_string(), "OBA".to_string());
    map.insert("Jonah".to_string(), "JON".to_string());
    map.insert("Micah".to_string(), "MIC".to_string());
    map.insert("Nahum".to_string(), "NAM".to_string());
    map.insert("Habakkuk".to_string(), "HAB".to_string());
    map.insert("Zephaniah".to_string(), "ZEP".to_string());
    map.insert("Haggai".to_string(), "HAG".to_string());
    map.insert("Zechariah".to_string(), "ZEC".to_string());
    map.insert("Malachi".to_string(), "MAL".to_string());
    map.insert("Matthew".to_string(), "MAT".to_string());
    map.insert("Mark".to_string(), "MRK".to_string());
    map.insert("Luke".to_string(), "LUK".to_string());
    map.insert("John".to_string(), "JHN".to_string());
    map.insert("Acts".to_string(), "ACT".to_string());
    map.insert("Romans".to_string(), "ROM".to_string());
    map.insert("1 Corinthians".to_string(), "1CO".to_string());
    map.insert("2 Corinthians".to_string(), "2CO".to_string());
    map.insert("Galations".to_string(), "GAL".to_string());
    map.insert("Ephesians".to_string(), "EPH".to_string());
    map.insert("Philippians".to_string(), "PHP".to_string());
    map.insert("Colossians".to_string(), "COL".to_string());
    map.insert("1 Thessalonians".to_string(), "1TH".to_string());
    map.insert("2 Thessalonians".to_string(), "2TH".to_string());
    map.insert("1 Timothy".to_string(), "1TI".to_string());
    map.insert("2 Timothy".to_string(), "2TI".to_string());
    map.insert("Titus".to_string(), "TIT".to_string());
    map.insert("Philemon".to_string(), "PHM".to_string());
    map.insert("Hebrews".to_string(), "HEB".to_string());
    map.insert("James".to_string(), "JAS".to_string());
    map.insert("1 Peter".to_string(), "1PE".to_string());
    map.insert("2 Peter".to_string(), "2PE".to_string());
    map.insert("1 John".to_string(), "1JN".to_string());
    map.insert("2 John".to_string(), "2JN".to_string());
    map.insert("3 John".to_string(), "3JN".to_string());
    map.insert("Jude".to_string(), "JUD".to_string());
    map.insert("Revelation".to_string(), "REV".to_string());
    map
});

pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn new(title: String, bible_doc: &roxmltree::Document) -> anyhow::Result<Self> {
        let title_id = BOOK_TITLE_ID
            .get(&title)
            .context(format!("{} is not book of the Bible", &title))?;
        let full_book_node: Node = bible_doc
            .descendants()
            .find(|node| node.has_tag_name(BOOK_TAG) && node.attribute(ID_TAG) == Some(title_id))
            .context(format!("cannot find the book of {}", &title))?;

        let mut book_struct = Book {
            title,
            chapters: Vec::new(),
        };
        book_struct.make_chapters(full_book_node)?;

        Ok(book_struct)
    }

    fn is_content(v: &Node) -> bool {
        v.has_tag_name(WORD_TAG) || v.is_text()
    }
    fn is_verse_tag(v: &Node) -> bool {
        v.has_attribute(ID_TAG) && v.has_tag_name(VERSE_TAG)
    }
    fn is_chapter_tag(child: &Node) -> bool {
        child.has_tag_name(CHPT_NUM_TAG)
            || child.has_tag_name(ALT_CHPT_NUM_TAG_1)
            || child.has_tag_name(ALT_CHPT_NUM_TAG_2)
    }

    fn is_paragraph_tag(child: &Node) -> bool {
        child.has_tag_name(P_PARA_TAG) || child.has_tag_name(Q_PARA_TAG)
    }
    fn make_chapters(&mut self, full_book: Node) -> anyhow::Result<()> {
        for child in full_book.children() {
            if Book::is_chapter_tag(&child) {
                let num = child
                    .attribute(ID_TAG)
                    .context("no ID tag on chapter")?
                    .parse::<u32>()?;

                //make a chapter of the book and then start working on it
                let chapter = Chapter {
                    number: num,
                    paragraphs: Vec::new(),
                };
                self.chapters.push(chapter);
                continue;
            }

            //find a paragraph node and we can start filling the chapter text
            if Book::is_paragraph_tag(&child) {
                let mut pgh = Paragraph { verses: Vec::new() };

                if child.has_children() {
                    //making a verse here means the paragraph is 1 verse long and contains all of
                    //the verses for that paragraph under the last verse of the paragraph

                    //make a new verse when we come across a verse tag and add it to the paragraph
                    //we append to the paragraph so we always update the last verse in the paragraph
                    //NOTE: where the verse meta data is (vs_num and content)
                    for v in child.children() {
                        if Book::is_verse_tag(&v) {
                            let mut new_verse = Verse::new(0, "");
                            new_verse.number =
                                v.attribute(ID_TAG).context("no verse ID")?.parse::<u32>()?;
                            pgh.verses.push(new_verse);
                            continue;
                        }

                        if Book::is_content(&v) {
                            //find the most recent verse
                            let verse_opt = pgh.verses.last_mut();

                            if let Some(most_recent_verse) = verse_opt {
                                //start adding contents to the verse
                                if let Some(t) = v.text() {
                                    //we don't want to add '\n' after each <w/> tag
                                    if t.contains(NEW_LN) {
                                        most_recent_verse.contents.push_str(&t.replace("\n", " "));
                                    } else {
                                        most_recent_verse.contents.push_str(t);
                                    }
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                }

                // dbg!(pgh.to_owned());
                if let Some(c) = self.chapters.iter_mut().last() {
                    c.paragraphs.push(pgh);
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.title.to_uppercase())?;
        for ch in &self.chapters {
            write!(f, "{}\n\n", ch)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bible_as_str, get_path_to_bible_file};
    use crate::{download_bible, Config};
    use rstest::*;

    #[fixture]
    fn book_fixture() -> Book {
        download_bible(&Config::default()).unwrap();
        let bible_str = bible_as_str(get_path_to_bible_file(&Config::default()).unwrap()).unwrap();
        let bible_doc = roxmltree::Document::parse(&bible_str).unwrap();
        Book::new("Genesis".to_string(), &bible_doc).unwrap()
    }

    #[rstest]
    fn new_book(book_fixture: Book) {
        assert_eq!(book_fixture.chapters.len(), 50);

        for chpt in book_fixture.chapters {
            assert_ne!(chpt.paragraphs.len(), 0);
            for para in chpt.paragraphs {
                for i in 0..para.verses.len() {
                    if i != para.verses.len() - 1 {
                        let v = &para.verses[i];
                        let v_next = &para.verses[i + 1];
                        assert!(v_next.number - v.number == 1);
                    }
                }
            }
        }
    }

    #[rstest]
    #[ignore]
    fn display_book(book_fixture: Book) {
        print!("{}", book_fixture);
    }
}
