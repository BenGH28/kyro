use serde::{Deserialize, Serialize};
use std::process::Command;

fn download_bible(language: &str, version: &str) {
	let url: String = format!(
		r#"https://raw.githubusercontent.com/gratis-bible/bible/master/{}/{}.xml"#,
		language, version
	);

	Command::new("curl")
		.arg("-O")
		.arg(url)
		.output()
		.expect("curl failed to start");
}

#[derive(Debug, Deserialize, Serialize)]
struct MyConfig {
	language: String,
	version: String,
}

impl Default for MyConfig {
	fn default() -> Self {
		MyConfig {
			language: "english".to_string(),
			version: "asv".to_string(),
		}
	}
}

fn main() -> Result<(), confy::ConfyError> {
	let cfg: MyConfig = confy::load("kyro")?;
	dbg!(cfg);
	Ok(())
}
