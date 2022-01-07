use std::ffi::OsString;
use std::vec::Vec;

use anyhow::Context;
use roxmltree::Node;

use crate::bible::chapter::Chapter;
use crate::bible::passage::{Passage, Point};
use crate::Config;

const CHPT_NUM_TAG: &str = "c";
const ALT_CHPT_NUM_TAG_1: &str = "ca";
const ALT_CHPT_NUM_TAG_2: &str = "cp";
const BOOK_TAG: &str = "book";

pub struct Book {
    pub title: String,
    pub entry_point: Point,
    pub end_point: Point,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn new(title: String, entry_point: Point, end_point: Point) -> Self {
        Self {
            title,
            entry_point,
            end_point,
            chapters: Vec::new(),
        }
    }

    ///this will make populate the chapters in the book
    pub fn init(&mut self, config: &Config, bible_doc: &roxmltree::Document) -> anyhow::Result<()> {
        let full_book: Node = bible_doc
            .root()
            .children()
            .find(|node| node.has_tag_name(BOOK_TAG))
            .context("there is no tag by that name")?;

        assert!(full_book.has_tag_name(BOOK_TAG));
        Ok(self.make_chapters(config, full_book)?)
    }

    fn make_chapters(&mut self, config: &Config, full_book: Node) -> anyhow::Result<()> {
        //find all the chapters in the the book and put them into the vector
        let chapter_node: Node = full_book
            .descendants()
            .find(|node| {
                node.tag_name().name() == CHPT_NUM_TAG
                    || node.tag_name().name() == ALT_CHPT_NUM_TAG_1
                    || node.tag_name().name() == ALT_CHPT_NUM_TAG_2
            })
            .context("no chapter tags available")?;
        let next_node = chapter_node.next_sibling();

        //create chapter structs
        if let Some(s) = chapter_node.text() {
            let chapter = Chapter {
                number: s
                    .parse::<u32>()
                    .context("did not find chapter number from xml node")?,
                entry_point: Point::new(),
                verses: Vec::new(),
            };

            //make all the verses for that chapter and then push to chapters vec
            self.chapters.push(chapter);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bible_as_str, get_path_to_bible_file};

    #[test]
    fn get_nodes() -> anyhow::Result<()> {
        let bible_str = bible_as_str(get_path_to_bible_file(&Config::default())?)?;
        let bible_doc = roxmltree::Document::parse(&bible_str)?;

        let full_book: Node = bible_doc
            .root()
            .descendants()
            .find(|node| node.has_tag_name(BOOK_TAG) && node.attribute("id") == Some("EXO"))
            .context("there is either no available tag or no available Book with that title")?;

        assert!(full_book.has_tag_name(BOOK_TAG));
        assert_eq!(full_book.attribute("id"), Some("EXO"));

        let chapter_node = full_book
            .descendants()
            .find(|n| n.has_tag_name(CHPT_NUM_TAG) && n.attribute("id") == Some("1"))
            .context("no chapter node found")?;
        assert!(chapter_node.attribute("id") == Some("1"));
        assert!(chapter_node.has_siblings());

        let sec_heading = chapter_node.next_sibling().unwrap().next_sibling().unwrap();

        println!("{:?}", sec_heading);

        assert_eq!(
            sec_heading.first_child().unwrap().text().unwrap(),
            "Blessing during Bondage in Egypt\n"
        );

        Ok(())
    }
}
