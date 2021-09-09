use kyro::*;
mod cli;
use cli::Command;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    //read the config
    let config = Config::get_config()?;
    if let Some(lang_code) = config.get_language_code() {
        download_bible(&lang_code, &config)?;

        let args = Command::from_args();
        args.run()
        //the config is good so we now have to work with the cli
    }
    Ok(())
}
