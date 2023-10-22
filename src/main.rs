use anyhow::{Context, Result};

mod actions;
mod archive;
mod compression;
mod config;
mod tar_archive;

use actions::{Actions, SubActions};
use archive::Archive;
use compression::Compression;
use config::Config;
use tar_archive::{create_tar_archive, extract_tar_archive};

fn main() -> Result<()> {
    let actions: Actions = argh::from_env();

    match actions.command {
        SubActions::Push(args) => {
            let config = Config::load()?;
            let archive = Archive::new(config)?;
            archive.push(&args.target)?
        }
        SubActions::Pull(args) => {
            let config = Config::load()?;
            let archive = Archive::new(config)?;
            archive.pull(&args.target)?
        }
        SubActions::List(_) => {
            let config = Config::load()?;
            let archive = Archive::new(config)?;
            archive.list()?
        }
        SubActions::DefaultConfig(_) => {
            println!("{}", toml::to_string(&Config::default()).unwrap())
        }
    }

    Ok(())
}
