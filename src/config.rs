use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	language: String,
	version: String,
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
