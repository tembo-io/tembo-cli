use crate::{cli::docker::Docker, Result};
use clap::{ArgMatches, Command};

// Create init subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("delete").about("Deletes database instance locally & on tembo cloud")
}

pub fn execute(_args: &ArgMatches) -> Result<()> {
    match Docker::stop_remove("tembo-pg") {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}
