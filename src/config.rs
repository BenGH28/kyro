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

	pub fn get_language_code(&self) -> Option<String> {
		let key = self.language.to_owned();
		let language_dict: HashMap<String, String> = Config::get_language_dict();
		let code = language_dict.get(&key);
		code.map(|c| c.to_owned())
	}

	pub fn get_file_name(&self) -> String {
		self.version.to_owned() + ".xml"
	}
}

#[cfg(test)]
mod tests {
	use super::Config;
	#[test]
	fn test_get_config() {
		let conf = Config::default();
		assert_eq!(conf.language, "english".to_owned());
		assert_eq!(conf.version, "asv".to_owned());
	}

	#[test]
	fn test_get_language_code() {
		let config = Config::default();
		let code = config.get_language_code();
		let code = match code {
			Some(code) => code,
			None => "".to_string(),
		};

		assert_eq!(code, "en".to_string());
	}
}
