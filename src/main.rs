use anyhow::Context as _;
use kyro::{download_bible, Command, Config};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    //read the config
    let config = Config::get_config()?;
    let lang_code = config.get_language_code().context("Unknown language")?;
    download_bible(&lang_code, &config)?;
    Command::from_args().run(&config)?;
    Ok(())
}
