use std::fmt;

use crate::bible::paragraph::Paragraph;
use crate::bible::passage::Point;

#[derive(Debug)]
pub struct Chapter {
    pub number: u32,
    pub entry_point: Point,
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

#[cfg(test)]
mod tests {
    use crate::bible::verse::Verse;

    use super::*;

    #[test]
    fn display_chapter() {
        let mut chpt = Chapter {
            number: 11,
            entry_point: Point {
                chpt: 11,
                verse: 34,
            },
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
}
