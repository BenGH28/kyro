mod config;

pub use config::Config;
use directories_next::ProjectDirs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

///Get the raw github URL from the config
pub fn get_bible_url(lang_code: &str, version: &str) -> String {
	let url: String = format!(
		r#"https://raw.githubusercontent.com/gratis-bible/bible/master/{}/{}.xml"#,
		lang_code, version
	);
	url
}

///Get the directory to store all the Bibles
pub fn get_data_dir() -> Option<PathBuf> {
	if let Some(proj_dirs) = ProjectDirs::from("", "BenGH28", "kyro") {
		let data_dir: &Path = proj_dirs.data_dir();
		return Some(data_dir.to_owned());
	}
	None
}

///Save a Bible in xml to a file on the computer under the XDG format (ie. $HOME/.local/share/kyro/)
pub fn save_to_pc(bible: &str, config: &Config) -> std::io::Result<()> {
	//get data_dir: $HOME/.local/share/kyro/
	if let Some(data_dir) = get_data_dir() {
		let file_path = data_dir
			.join(config.language.to_owned())
			.join(config.get_file_name());
		//get the path to the language sub directory (ie. $HOME/.local/share/kyro/english/)
		let language_dir = data_dir.join(config.language.to_owned());
		//if the directory doesn't exist then lets create it
		if !language_dir.is_dir() {
			std::fs::create_dir_all(language_dir)
				.expect("could not create kyro project data directory");
		}
		//if the file doesn't exist then we can write it
		if !file_path.is_file() {
			//create and write the Bible to a file
			let mut file = File::create(file_path).expect("could not write to file");
			file.write_all(bible.as_bytes())?;
		}
		Ok(())
	} else {
		panic!("could not determine the Project Directory path")
	}
}

///Get the text from the gratis-bible github account
pub fn get_bible_text(url: String) -> Result<String, reqwest::Error> {
	let bible = reqwest::blocking::get(url)?.text()?;
	Ok(bible)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_bible_text() {
		let config = Config::default();
		if let Some(code) = config.get_language_code() {
			let url = get_bible_url(&code, &config.version);
			let text = get_bible_text(url).unwrap();
			assert_ne!("", text);
		} else {
			panic!()
		};
	}
}
