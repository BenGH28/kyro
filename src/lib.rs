mod config;

use anyhow::Context;
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

///Download the Bible and save it on your computer
pub fn download_bible(lang_code: &str, config: &Config) -> anyhow::Result<()> {
    let url = get_bible_url(lang_code, &config.version);
    save_to_pc(&url, &config)?;
    Ok(())
}

///Save a Bible in xml to a file on the computer under the XDG format (ie. $HOME/.local/share/kyro/)
pub fn save_to_pc(url: &str, config: &Config) -> anyhow::Result<()> {
    //get data_dir: $HOME/.local/share/kyro/
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
    let file_path: PathBuf = data_dir.join(&config.language).join(config.get_file_name());
    //get the path to the language sub directory (ie. $HOME/.local/share/kyro/english/)
    let language_dir: PathBuf = data_dir.join(&config.language);
    //if the directory doesn't exist then lets create it
    if !language_dir.is_dir() {
        std::fs::create_dir_all(language_dir)
            .context("could not create kyro project data directory")?;
    }
    //if the file doesn't exist then we can write it
    if !file_path.is_file() {
        //create and write the Bible to a file
        let bible: String = get_bible_text(url)?;
        let mut file: File = File::create(file_path).context("could not write Bible to file")?;
        file.write_all(bible.as_bytes())?;
    }
    Ok(())
}

///Get the text from the gratis-bible github account
pub fn get_bible_text(url: &str) -> anyhow::Result<String> {
    let bible: String = reqwest::blocking::get(url)?.text()?;
    Ok(bible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bible_text() {
        let config = Config::default();
        let code = config.get_language_code().unwrap();
        let url: String = get_bible_url(&code, &config.version);
        let text: String = get_bible_text(&url).unwrap();
        assert_ne!("", text);
    }
}
