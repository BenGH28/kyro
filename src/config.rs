use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub language: String,
	pub version: String,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			language: "english".to_string(),
			version: "asv".to_string(),
		}
	}
}

impl Config {
	pub fn get_config() -> Result<Config, confy::ConfyError> {
		let conf: Config = confy::load("kyro")?;
		Ok(conf)
	}

	pub fn get_language_dict() -> HashMap<String, String> {
		let mut language_dict: HashMap<String, String> = HashMap::new();
		language_dict.insert("english".to_string(), "en".to_string());
		language_dict
	}

	pub fn get_language_code(config_lang: String) -> Option<String> {
		let language_dict: HashMap<String, String> = Config::get_language_dict();
		let code = language_dict.get(&config_lang);
		match code {
			Some(c) => Some(c.to_owned()),
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Config;
	#[test]
	fn test_get_config() {
		let conf: Config;
		if let Ok(i) = Config::get_config() {
			conf = i;
			assert_eq!(conf.language, "english".to_string());
			assert_eq!(conf.version, "asv".to_string());
		} else {
			panic!("config wasn't correct");
		}
	}
}
