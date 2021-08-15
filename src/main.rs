mod config;

use config::Config;
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

fn main() -> Result<(), confy::ConfyError> {
	let cfg: Config = confy::load("kyro")?;
	dbg!(cfg);
	Ok(())
}
