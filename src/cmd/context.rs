use std::error::Error;

use clap::ArgMatches;
use simplelog::*;

pub mod list;
pub mod set;

pub const CONTEXT_DEFAULT_TEXT: &str = "version = \"1.0\"

[[environment]]
name = 'local'
target = 'docker'

[[environment]]
name = 'prod'
target = 'tembo-cloud'
org_id = 'ORG_ID'
";

pub fn tembo_home_dir() -> String {
    let mut tembo_home = home::home_dir().unwrap().as_path().display().to_string();
    tembo_home.push_str("/.tembo");
    tembo_home
}

pub fn tembo_context_file_path() -> String {
    return tembo_home_dir() + &String::from("/context");
}

// handles all context command calls
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // execute the context subcommands
    let res = match args.subcommand() {
        Some(("list", sub_matches)) => list::execute(sub_matches),
        Some(("set", sub_matches)) => set::execute(sub_matches),

        _ => unreachable!(),
    };

    if res.is_err() {
        error!("{}", res.err().unwrap());

        // TODO: adding logging, log error
        std::process::exit(101);
    }

    Ok(())
}
