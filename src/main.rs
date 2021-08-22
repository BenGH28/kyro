use kyro::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let config = Config::get_config()?;
	if let Some(lang_code) = config.get_language_code() {
		let url = get_bible_url(&lang_code, &config.version);
		let bible = get_bible_text(url)?;
		save_to_pc(&bible, &config)?;
	}
	Ok(())
}
