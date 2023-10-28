// instance deploy command
use crate::cli::config::Config;
use anyhow::anyhow;
use clap::{Arg, ArgAction, ArgMatches, Command};
use simplelog::*;
use std::error::Error;

// example usage: tembo instance deploy -n my_local_instance
pub fn make_subcommand() -> Command {
    Command::new("deploy")
        .about("Command used to deploy a local instance to the Tembo cloud")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .action(ArgAction::Set)
                .required(true)
                .help("The name you want to use for this instance"),
        )
}

pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let config = Config::new(args, &Config::full_path(args));
    let name = args
        .get_one::<String>("name")
        .ok_or_else(|| anyhow!("Name is missing."))?;

    if config.instances.is_empty() {
        info!("No instances have been configured");
    } else {
        for instance in &config.instances {
            if instance.name.clone().unwrap().to_lowercase() == name.to_lowercase() {
                match instance.deploy(args) {
                    Ok(_) => info!(" instance deployed"),
                    Err(e) => warn!(" there was an error deploying the instance: {}", e),
                }
            }
        }
    }

    Ok(())
}
