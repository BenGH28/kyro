use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

use anyhow::Context;
use once_cell::sync::Lazy;
use roxmltree::Node;

use crate::bible::chapter::Chapter;
use crate::bible::passage::Point;

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
    pub entry_point: Point,
    pub end_point: Point,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn new(
        title: String,
        entry_point: Point,
        end_point: Point,
        bible_doc: &roxmltree::Document,
    ) -> anyhow::Result<Self> {
        let title_id = BOOK_TITLE_ID.get(&title).context("no book by that title")?;
        let full_book_node: Node = bible_doc
            .descendants()
            .find(|node| node.has_tag_name(BOOK_TAG) && node.attribute(ID_TAG) == Some(title_id))
            .context("there is no tag by that name")?;

        let mut book_struct = Book {
            title,
            entry_point,
            end_point,
            chapters: Vec::new(),
        };
        book_struct.make_chapters(full_book_node)?;

        Ok(book_struct)
    }

    fn make_chapters(&mut self, full_book: Node) -> anyhow::Result<()> {
        for child in full_book.children() {
            if child.has_tag_name(CHPT_NUM_TAG)
                || child.has_tag_name(ALT_CHPT_NUM_TAG_1)
                || child.has_tag_name(ALT_CHPT_NUM_TAG_2)
            {
                let num = child
                    .attribute(ID_TAG)
                    .context("no ID tag on chapter")?
                    .parse::<u32>()?;

                //make a chapter of the book and then start working on it
                let chapter = Chapter {
                    number: num,
                    entry_point: Point {
                        chpt: num,
                        verse: 1,
                    },
                    paragraphs: Vec::new(),
                };
                self.chapters.push(chapter);
                continue;
            }

            //find a paragraph node and we can start filling the chapter text
            if child.has_tag_name(P_PARA_TAG) || child.has_tag_name(Q_PARA_TAG) {
                // println!("trying to build a paragraph");
                //the new paragraph to add to the chapter
                let mut p = Paragraph { verses: Vec::new() };

                if child.has_children() {
                    //make a verse to add to the paragraph later
                    let mut verse = Verse {
                        number: 0,
                        contents: String::from(""),
                    };
                    for v in child.children() {
                        // make sure to set the verse number
                        if v.has_attribute(ID_TAG) {
                            verse.number =
                                v.attribute(ID_TAG).context("no verse ID")?.parse::<u32>()?;
                        }

                        // start building the contenst of the verse with <w/> tags and text nodes
                        // which follow the <w/> tags holding any punctuation for the grammar
                        if v.has_tag_name(WORD_TAG) || v.is_text() {
                            if let Some(t) = v.text() {
                                //we don't want to add '\n' after each <w/> tag
                                if t.contains(NEW_LN) {
                                    verse.contents.push_str(&t.replace("\n", " "));
                                } else {
                                    verse.contents.push_str(t);
                                }
                            }
                        }
                    }
                    p.verses.push(verse);
                }
                if let Some(c) = self.chapters.iter_mut().last() {
                    c.paragraphs.push(p);
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    #[test]
    #[ignore]
    fn get_nodes() -> anyhow::Result<()> {
        let book_title = "GEN";
        let bible_str = bible_as_str(get_path_to_bible_file(&Config::default())?)?;
        let bible_doc = roxmltree::Document::parse(&bible_str)?;

        let full_book: Node = bible_doc
            .descendants()
            .find(|node| node.has_tag_name(BOOK_TAG) && node.attribute(ID_TAG) == Some(book_title))
            .context("there is either no available tag or no available Book with that title")?;

        // println!("{:?}", full_book.descendants());

        assert!(full_book.has_tag_name(BOOK_TAG));
        assert_eq!(full_book.attribute(ID_TAG), Some(book_title));

        let chapter_node = full_book
            .descendants()
            .find(|n| n.has_tag_name(CHPT_NUM_TAG) && n.attribute(ID_TAG) == Some("1"))
            .context("no chapter node found")?;
        assert!(chapter_node.attribute(ID_TAG) == Some("1"));
        assert!(chapter_node.has_siblings());

        // println!("{:?}", chapter_node.descendants());

        let sec_heading = chapter_node.next_sibling().unwrap().next_sibling().unwrap();

        // println!("{:?}", sec_heading);

        assert_eq!(
            sec_heading.first_child().unwrap().text().unwrap(),
            "The Creation of the World\n"
        );

        assert!(sec_heading
            .next_sibling_element()
            .unwrap()
            .has_tag_name("p"));

        let paragraph = sec_heading.next_sibling_element().unwrap();
        for d in paragraph.descendants() {
            if d.is_element() {
                match d.text() {
                    Some(s) => println!("{:?}", s),
                    None => continue,
                }
            } else {
                d.text().unwrap();
            }
        }

        Ok(())
    }

    #[test]
    fn new_book() -> anyhow::Result<()> {
        download_bible(&Config::default())?;
        let bible_str = bible_as_str(get_path_to_bible_file(&Config::default())?)?;
        let entry_point = Point { chpt: 2, verse: 2 };
        let end_point = Point { chpt: 2, verse: 3 };
        let bible_doc = roxmltree::Document::parse(&bible_str)?;
        let book = Book::new("Exodus".to_string(), entry_point, end_point, &bible_doc)?;

        assert_ne!(book.chapters.len(), 0);

        for chpt in book.chapters {
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

        Ok(())
    }

    #[test]
    fn display_book() -> anyhow::Result<()> {
        let title = "1 John".to_string();
        download_bible(&Config::default())?;
        let bible_str = bible_as_str(get_path_to_bible_file(&Config::default())?)?;
        let bible_doc = roxmltree::Document::parse(&bible_str)?;
        let book = Book::new(title, Point::new(), Point::new(), &bible_doc)?;
        print!("{}", book);
        Ok(())
    }
}
