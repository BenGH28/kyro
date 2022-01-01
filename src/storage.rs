pub use crate::config::Config;
use anyhow::Context as _;
use bytes::Bytes;
use directories_next::ProjectDirs;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

///Get the raw github URL from the config
pub fn get_bible_url() -> String {
    let url = "https://ebible.org/Scriptures/engnet_usfx.zip".to_string();
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
pub fn download_bible() -> anyhow::Result<()> {
    let url = get_bible_url();
    save_to_pc(&url)?;
    Ok(())
}

///Save a Bible in xml to a file on the computer under the XDG format (ie. $HOME/.local/share/kyro/)
pub fn save_to_pc(url: &str) -> anyhow::Result<()> {
    //get data_dir: $HOME/.local/share/kyro/
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;

    //the full path to the extracted archive (ie. $HOME/.local/share/kyro/engnet_usfx/)
    let unzip_dir: PathBuf = data_dir.join("engnet_usfx");

    //the full path to the extracted archive (ie. $HOME/.local/share/kyro/engnet_usfx/engnet_usfx.xml)
    let file_path: PathBuf = unzip_dir.join("engnet_usfx.xml");
    let file_zip_path: PathBuf = data_dir.join("engnet_usfx.zip");

    //if the directory doesn't exist then lets create it
    if !unzip_dir.is_dir() {
        std::fs::create_dir_all(unzip_dir)
            .context("could not create kyro project data directory")?;
    }

    //if the file doesn't exist then we can write it
    if !file_path.is_file() {
        //create and write the Bible to a file
        let bible_zip: Bytes = get_bible_zip(url)?;
        // unzip(&bible_zip)?;
        let mut file: File = File::create(file_zip_path).context("could not write Bible to file")?;
        file.write_all(&bible_zip)?;
    }
    Ok(())
}

fn unzip(zipped: &Bytes)-> anyhow::Result<()>{
    Ok(())
}
pub fn read_bible_from_file(path: &str) -> anyhow::Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

///get the absolute path to the bible xml file
pub fn get_path_to_bible_file(config: &Config) -> anyhow::Result<OsString> {
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
    let file_path: OsString = data_dir
        .join(&config.language)
        .join(config.get_file_name())
        .into_os_string();
    Ok(file_path)
}

///Get the text from the gratis-bible github account
pub fn get_bible_zip(url: &str) -> anyhow::Result<Bytes> {
    let bible: Bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(bible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_save_to_pc() -> anyhow::Result<()>{
        let url = &get_bible_url();
        save_to_pc(url)?;

        let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
        let file_zip_path: PathBuf = data_dir.join("engnet_usfx.zip");

        assert!(file_zip_path.is_file());

        Ok(())
    }

    #[test]
    fn test_get_bible_text() {
        let url: String = get_bible_url();
        let text: Bytes = get_bible_zip(&url).unwrap();
        assert_ne!("", text);
    }
}
