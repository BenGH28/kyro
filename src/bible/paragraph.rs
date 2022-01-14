use std::fmt;

use crate::bible::verse::Verse;

#[derive(Debug)]
pub struct Paragraph {
    pub verses: Vec<Verse>,
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in &self.verses {
            write!(f, "{} ", v)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_paragraph() {
        let mut p = Paragraph { verses: Vec::new() };

        let expected = r#"[34] "Where have you put him?" He asked. "Come and see, Lord," they answered. [35] Jesus wept. "#.to_string();

        let j_1134 = Verse {
            number: 34,
            contents: r#""Where have you put him?" He asked. "Come and see, Lord," they answered."#
                .to_string(),
        };
        let j_1135 = Verse {
            number: 35,
            contents: "Jesus wept.".to_string(),
        };
        p.verses.push(j_1134);
        p.verses.push(j_1135);

        assert_eq!(format!("{}", p), expected);
    }
}
