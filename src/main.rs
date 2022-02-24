use kyro::{download_bible, Command, Config};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let config = Config::get_config()?;
    download_bible(&config)?;
    Command::from_args().run(&config)?;
    Ok(())
}
