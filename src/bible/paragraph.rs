use std::fmt;
use std::fmt::Write as FmtWrite;
use textwrap::{fill, termwidth, wrap_algorithms::Penalties, Options, WrapAlgorithm};

use crate::bible::verse::Verse;

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub verses: Vec<Verse>,
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut all_vs = String::new();
        for v in &self.verses {
            write!(&mut all_vs, "[{}] {} ", v.number, v.contents)?;
        }

        //thanks Steve!!
        let opts =
            Options::new(termwidth()).wrap_algorithm(WrapAlgorithm::OptimalFit(Penalties::new()));
        write!(f, "{}", fill(&all_vs, opts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_paragraph() {
        let mut p = Paragraph { verses: Vec::new() };

        let opts =
            Options::new(termwidth()).wrap_algorithm(WrapAlgorithm::OptimalFit(Penalties::new()));
        let mut expected = r#"[34] "Where have you put him?" He asked. "Come and see, Lord," they answered. [35] Jesus wept."#.to_string();
        expected = fill(&expected, opts);

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

        let result = format!("{}", p);

        assert_eq!(result, expected);
    }
}
