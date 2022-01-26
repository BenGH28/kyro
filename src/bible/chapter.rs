use std::fmt;

use anyhow::Context;

use super::paragraph::Paragraph;
use super::passage;

#[derive(Debug)]
pub struct Chapter {
    pub number: u32,
    pub entry_vs: u32,
    pub paragraphs: Vec<Paragraph>,
}

impl fmt::Display for Chapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHAPTER {}", self.number)?;
        for p in &self.paragraphs {
            writeln!(f, "{}", p)?;
        }
        Ok(())
    }
}

impl passage::Navigate for Chapter {
    type Output = Paragraph;

    fn forward(&mut self) -> Option<&Self::Output> {
        //TODO
        None
    }

    fn backward(&mut self) -> Option<&Self::Output> {
        //TODO
        None
    }

    fn begin(&self) -> anyhow::Result<&Self::Output> {
        self.paragraphs
            .iter()
            .find(|p| {
                let result = p.verses.iter().find(|v| v.number == self.entry_vs);
                match result {
                    Some(v) => v.number == self.entry_vs,
                    None => false,
                }
            })
            .context(format!(
                "could not find verse {} in chapter {}",
                self.entry_vs, self.number
            ))
    }

    fn end(&self) -> Option<&Self::Output> {
        //TODO:: may need to sort this out later...
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bible::{passage::Navigate, verse::Verse};
    use rstest::*;

    #[test]
    fn display_chapter() {
        let mut chpt = Chapter {
            number: 11,
            entry_vs: 34,
            paragraphs: Vec::new(),
        };

        let mut p1 = Paragraph { verses: Vec::new() };
        let mut p2 = Paragraph { verses: Vec::new() };

        let expected = "CHAPTER 11\n[34] \"Where have you put him?\" He asked. \"Come and see, Lord,\" they answered. [35] Jesus wept. \n[36] So the Jews said, \"See how he loved him!\" \n".to_string();

        let j_1134 = Verse {
            number: 34,
            contents: r#""Where have you put him?" He asked. "Come and see, Lord," they answered."#
                .to_string(),
        };

        let j_1135 = Verse {
            number: 35,
            contents: "Jesus wept.".to_string(),
        };
        p1.verses.push(j_1134);
        p1.verses.push(j_1135);

        let j_1136 = Verse {
            number: 36,
            contents: r#"So the Jews said, "See how he loved him!""#.to_string(),
        };

        //verse is a new paragraph just for testing purposes
        p2.verses.push(j_1136);

        chpt.paragraphs.push(p1);
        chpt.paragraphs.push(p2);

        assert_eq!(format!("{}", chpt), expected);
    }

    #[fixture]
    fn chapter_fixture() -> Chapter {
        let mut ch = Chapter {
            number: 3,
            entry_vs: 4,
            paragraphs: Vec::new(),
        };

        let mut p1 = Paragraph { verses: Vec::new() };
        let mut p2 = Paragraph { verses: Vec::new() };

        p1.verses
            .push(Verse::new(1, "here is some content for v1 of paragraph 1"));
        p2.verses
            .push(Verse::new(4, "here is some content for v4 of paragraph 2"));

        ch.paragraphs.push(p1);
        ch.paragraphs.push(p2);

        ch
    }

    #[rstest]
    fn begin_chapter(chapter_fixture: Chapter) {
        let expected = Verse::new(4, "here is some content for v4 of paragraph 2");
        let p = chapter_fixture.begin().unwrap();
        let v = &p.verses[0];

        assert_eq!(v.number, expected.number);
        assert_eq!(v.contents, expected.contents);
    }
}
