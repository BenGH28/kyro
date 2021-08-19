mod config;

use config::Config;

fn get_bible_url(language: String, version: String) -> String {
	let url: String = format!(
		r#"https://raw.githubusercontent.com/gratis-bible/bible/master/{}/{}.xml"#,
		language, version
	);
	url
}

fn download_bible(url: String) -> Result<String, reqwest::Error> {
	let bible = reqwest::blocking::get(url)?.text();
	match bible {
		Ok(text) => return Ok(text),
		Err(e) => return Err(e),
	}
}

fn main() -> Result<(), reqwest::Error> {
	if let Ok(conf) = Config::get_config() {
		if let Some(lang_code) = Config::get_language_code(conf.language) {
			let url = get_bible_url(lang_code, conf.version);
			let bible = download_bible(url);
			match bible {
				Ok(text) => println!("{:?}", text),
				Err(e) => println!("{:?}", e),
			}
		}
	}

	Ok(())
}
