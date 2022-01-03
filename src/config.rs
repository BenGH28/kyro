use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub language: Language,
    pub version: BibleVersion,
}

#[derive(Hash, Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd)]
pub enum Language {
    English,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum BibleVersion {
    Net,
}

impl fmt::Display for BibleVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            language: Language::English,
            version: BibleVersion::Net,
        }
    }
}

impl Config {
    pub fn get_config() -> anyhow::Result<Config> {
        let conf: Config = confy::load("kyro")?;
        Ok(conf)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{BibleVersion, Language};

    use super::Config;
    #[test]
    fn test_get_config() {
        let conf = Config::get_config().unwrap();
        assert_eq!(conf.language, Language::English);
        assert_eq!(conf.version, BibleVersion::Net);
    }

    #[test]
    fn test_language_display() {
        let en: Language = Language::English;

        let en_string: String = en.to_string();

        assert_eq!(en_string, "English");
    }
}
