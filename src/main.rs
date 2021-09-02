use kyro::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::get_config()?;
    if let Some(lang_code) = config.get_language_code() {
        download_bible(&lang_code, &config)?
    }
    Ok(())
}
