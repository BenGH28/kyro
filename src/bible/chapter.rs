use std::fmt;

use super::paragraph::Paragraph;

#[derive(Debug)]
pub struct Chapter {
    pub number: u32,
    pub paragraphs: Vec<Paragraph>,
}

impl fmt::Display for Chapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHAPTER_{}", self.number)?;
        for p in &self.paragraphs {
            writeln!(f, "{}\n", p)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bible::verse::Verse;
    use rstest::*;

    #[test]
    fn display_chapter() {
        let mut chpt = Chapter {
            number: 11,
            paragraphs: Vec::new(),
        };

        let mut p1 = Paragraph { verses: Vec::new() };
        let mut p2 = Paragraph { verses: Vec::new() };

        let expected = "CHAPTER_11\n[34] \"Where have you put him?\" He asked. \"Come and see, Lord,\" they answered. [35] Jesus wept.\n\n[36] So the Jews said, \"See how he loved him!\"\n\n".to_string();

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
}
