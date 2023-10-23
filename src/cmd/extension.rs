use clap::ArgMatches;
use simplelog::*;
use std::error::Error;

pub mod enable;
pub mod install;
pub mod list;

// handles all extension command calls
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // execute the instance subcommands
    let res = match args.subcommand() {
        Some(("list", sub_matches)) => list::execute(sub_matches),
        Some(("install", sub_matches)) => install::execute(sub_matches),
        Some(("enable", sub_matches)) => enable::execute(sub_matches),
        _ => unreachable!(),
    };

    if res.is_err() {
        error!("{}", res.err().unwrap());

        // TODO: adding logging, log error
        std::process::exit(101);
    }

    Ok(())
}
