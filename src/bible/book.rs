use std::vec::Vec;

use anyhow::Context;
use roxmltree::Node;

use crate::bible::chapter::Chapter;
use crate::bible::passage::{Passage, Point};

use super::paragraph::Paragraph;
use super::verse::Verse;

const CHPT_NUM_TAG: &str = "c";
const ALT_CHPT_NUM_TAG_1: &str = "ca";
const ALT_CHPT_NUM_TAG_2: &str = "cp";
const BOOK_TAG: &str = "book";
const P_PARA_TAG: &str = "p";
const Q_PARA_TAG: &str = "q";
const VERSE_BEGIN_TAG: &str = "v";
const VERSE_END_TAG: &str = "ve";
const WORD_TAG: &str = "w";
const BLANKLN_TAG: &str = "b";
const ID_TAG: &str = "id";
const NEW_LN: &str = "\n";

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
        let full_book_node: Node = bible_doc
            .descendants()
            .find(|node| node.has_tag_name(BOOK_TAG))
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
                                if let Some(s) = t.strip_suffix(NEW_LN) {
                                    verse.contents += s;
                                } else {
                                    verse.contents += t;
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
}
