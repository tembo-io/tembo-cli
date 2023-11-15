// context list command
use crate::{cli::config::Config, cmd::{auth::info, context::{tembo_context_file_path, Context}}};
use clap::{ArgMatches, Command};
use simplelog::*;
use std::{error::Error, fs};

// example usage: tembo context create -t oltp -n my_app_db -p 5432
pub fn make_subcommand() -> Command {
    Command::new("list").about("Command used to list local contexts")
}

pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let filename = tembo_context_file_path();

    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(e) => {
            panic!("Couldn't read config file {}: {}", filename, e);
        }
    };

    let data: Context = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            panic!("Unable to load data from `{}`", e);
        }
    };

    info!("{}", contents);
    
    info!("{}", data.version);
    
    Ok(())
}
