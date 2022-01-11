pub use crate::config::Config;
use crate::config::{BibleVersion, Language};
use anyhow::Context as _;
use bytes::Bytes;
use directories_next::ProjectDirs;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

static CODES: Lazy<HashMap<Language, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Language::English, "eng".to_string());
    map
});

const XML: &str = ".xml";
const ZIP: &str = ".zip";
const USFX: &str = "_usfx";
const GITHUB_USER: &str = "BenGH28";
const KYRO: &str = "kyro";

///Get the raw github URL from the config
pub fn get_bible_url(config: &Config) -> String {
    match config.language {
        Language::English => match config.version {
            BibleVersion::Net => "https://ebible.org/Scriptures/engnet_usfx.zip".to_string(),
        },
    }
}

///Get the directory to store all the Bibles
pub fn get_data_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", GITHUB_USER, KYRO) {
        let data_dir: &Path = proj_dirs.data_dir();
        return Some(data_dir.to_owned());
    }
    None
}

///Download the Bible and save it on your computer
pub fn download_bible(config: &Config) -> anyhow::Result<()> {
    let url = get_bible_url(config);
    save_to_pc(&url, config)?;
    Ok(())
}

pub fn usfx_file(config: &Config, extension: &str) -> anyhow::Result<String> {
    let lang = &config.language;
    let version: String = config.version.to_string().to_ascii_lowercase();

    let code: String = CODES
        .get(lang)
        .context("cannot find specified language")?
        .to_string();

    //engnet_usfx.xml | .zip
    let fname = code + &version + USFX + extension;
    Ok(fname)
}

///Save a Bible in xml to a file on the computer under the XDG format (ie. $HOME/.local/share/kyro/)
pub fn save_to_pc(url: &str, config: &Config) -> anyhow::Result<()> {
    //get data_dir: $HOME/.local/share/kyro/
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;

    //the full path to the extracted archive (ie. $HOME/.local/share/kyro/English/Net/)
    let version_dir: PathBuf = data_dir
        .join(config.language.to_string())
        .join(config.version.to_string());

    //the full path to the extracted archive (ie. $HOME/.local/share/kyro/English/Net/engnet_usfx.xml)
    let file_path: PathBuf = version_dir.join(usfx_file(config, XML)?);

    //the full path to the archive (ie. $HOME/.local/share/kyro/English/Net/engnet_usfx.zip)
    let file_zip_path: PathBuf = version_dir.join(usfx_file(config, ZIP)?);

    //if the directory doesn't exist then lets create it
    if !version_dir.is_dir() {
        fs::create_dir_all(&version_dir).context("cannot creat directory for storing Bibles")?;
    }

    //if the file doesn't exist then we can write it
    if !file_path.is_file() {
        let bible_zip: Bytes = get_bible_zip(url)?;
        let mut file: File =
            File::create(&file_zip_path).context("could not write Bible to file")?;
        file.write_all(&bible_zip)?;
        unzip(&file_zip_path, &version_dir).context("unzipping has failed some how")?;
    }
    Ok(())
}

fn unzip(zipped_file_path: &Path, dest_dir: &Path) -> anyhow::Result<()> {
    let zipped_file = File::open(zipped_file_path).context("cannot open zip file")?;
    let mut archive = zip::ZipArchive::new(zipped_file).context("cannot make an archive reader")?;

    let result = archive
        .extract(dest_dir)
        .context("cannot extract the files")?;
    Ok(result)
}

pub fn bible_as_str(path: PathBuf) -> anyhow::Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

///get the absolute path to the bible xml file
pub fn get_path_to_bible_file(config: &Config) -> anyhow::Result<PathBuf> {
    let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
    let file_path: PathBuf = data_dir
        .join(&config.language.to_string())
        .join(&config.version.to_string())
        .join(usfx_file(config, XML)?);
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
    fn test_get_bible_file_path() -> anyhow::Result<()> {
        let actual_path = get_path_to_bible_file(&Config::default())?;
        let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
        let expected_path = data_dir.join("English").join("Net").join("engnet_usfx.xml");

        assert_eq!(actual_path, expected_path);

        Ok(())
    }
    #[test]
    #[ignore]
    fn test_download() -> anyhow::Result<()> {
        download_bible(&Config::default())?;

        let data_dir: PathBuf = get_data_dir().context("couldn't determine data dir path")?;
        let extracted_file: PathBuf = data_dir.join("English").join("Net").join("engnet_usfx.xml");

        assert!(extracted_file.is_file());
        Ok(())
    }

    #[test]
    fn test_get_zip() {
        let url: String = get_bible_url(&Config::default());
        let text: Bytes = get_bible_zip(&url).unwrap();
        assert_ne!("", text);
    }
    #[test]
    fn test_usfx_filename() -> anyhow::Result<()> {
        let mut fname = usfx_file(&Config::default(), XML)?;
        assert_eq!(fname, String::from("engnet_usfx.xml"));
        fname = usfx_file(&Config::default(), ZIP)?;
        assert_eq!(fname, String::from("engnet_usfx.zip"));
        Ok(())
    }
}
