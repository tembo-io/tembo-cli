use clap::{ArgMatches, Command};
use std::{error::Error, fs};

use crate::{cli::context::Context, cmd::context::tembo_context_file_path};

pub fn make_subcommand() -> Command {
    Command::new("list").about("Command used to list context")
}

pub fn execute(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let filename = tembo_context_file_path();

    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(e) => {
            panic!("Couldn't read context file {}: {}", filename, e);
        }
    };

    let data: Context = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            panic!("Unable to load data. Error: `{}`", e);
        }
    };

    // TODO: Improve formatting
    println!("Name           Target         Org ID         Set");
    println!("-------------- -------------- -------------- --------------");

    for e in data.environment {
        let mut org_id = String::new();
        let mut set = false;
        if !e.org_id.is_none() {
            org_id = e.org_id.unwrap();
        }
        if !e.set.is_none() {
            set = e.set.unwrap();
        }
        println!(
            "{}           {}        {:?}           {:?}",
            e.name, e.target, org_id, set
        );
    }

    Ok(())
}
