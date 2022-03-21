use std::fmt;

use textwrap::{fill, termwidth, wrap_algorithms::Penalties, Options, WrapAlgorithm};

#[derive(Clone, Debug)]
pub struct Verse {
    pub number: u32,
    pub contents: String,
}

impl Verse {
    pub fn new(vs_num: u32, content: &str) -> Self {
        Self {
            number: vs_num,
            contents: content.to_string(),
        }
    }
}

impl fmt::Display for Verse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = format!("[{}] {}", self.number, self.contents);
        let opts =
            Options::new(termwidth()).wrap_algorithm(WrapAlgorithm::OptimalFit(Penalties::new()));
        write!(f, "{}", fill(&v, opts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_verse() {
        let v = Verse {
            number: 35,
            contents: "Jesus wept.".to_string(),
        };
        assert_eq!(format!("{}", v), "[35] Jesus wept.");
    }
}
