use std::fmt;

#[derive(Clone, Debug)]
pub struct Verse {
    pub number: u32,
    pub contents: String,
}

impl fmt::Display for Verse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.number, self.contents)
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
