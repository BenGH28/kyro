use kyro::{download_bible, Command, Config};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    //read the config
    let config = Config::get_config()?;
    download_bible()?;
    Command::from_args().run(&config)?;
    Ok(())
}
