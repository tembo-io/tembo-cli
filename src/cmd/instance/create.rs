//! instance create command

use crate::cli::config::Config;
use crate::cli::docker::Docker;
use crate::cli::instance::{EnabledExtension, InstalledExtension, Instance};
use crate::cli::stacks;
use crate::cli::stacks::Stacks;
use crate::Result;
use anyhow::bail;
use chrono::prelude::*;
use clap::{Arg, ArgAction, ArgMatches, Command};
use simplelog::*;

// example usage: tembo instance create -t oltp -n my_app_db -p 5432
pub fn make_subcommand() -> Command {
    Command::new("create")
        .about("Command used to create a local instance")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .action(ArgAction::Set)
                .required(false)
                .default_value("standard")
                .help("The name of a Tembo stack type to use"),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .action(ArgAction::Set)
                .required(true)
                .help("The name you want to use for this instance"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .required(false)
                .default_value("5432")
                .help("The port number you want to use for this instance (default is 5432)"),
        )
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let matches = args;

    // ensure the stack type provided is valid, if none given, default to the standard stack
    if let Ok(_stack) = stacks::define_stack(matches) {
        // make sure requirements are met
        match check_requirements() {
            Ok(_) => info!("Docker was found and appears running"),
            Err(e) => {
                error!("{}", e);
                return Err(e);
            }
        }

        // TODO: make sure the instance name is valid
        // unique and API will respond with 4xx when instance name starts
        // or ends with non-alpha numeric character

        match persist_instance_config(matches) {
            Ok(_) => info!("Instance config persisted in config file"),
            Err(e) => {
                error!("{}", e);
                return Err(e);
            }
        }

        info!("Instance configuration created, you can start the instance using the command 'tembo start -i <name>'");
    } else {
        bail!("- Given Stack type is not valid");
    }

    Ok(())
}

fn check_requirements() -> Result<()> {
    Docker::installed_and_running()
}

fn persist_instance_config(matches: &ArgMatches) -> Result<()> {
    let path = Config::full_path(matches);
    let mut config = Config::new(matches, &path); // calls init and writes the file

    let r#type = matches.get_one::<String>("type").unwrap();
    let name = matches.get_one::<String>("name").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    let mut instance = Instance {
        name: Some(name.to_string()),
        r#type: Some(r#type.to_string()),
        port: Some(port.to_string()),
        created_at: Some(Utc::now()),
        version: None,
        installed_extensions: vec![],
        enabled_extensions: vec![],
        databases: vec![],
    };

    let stacks: Stacks = stacks::define_stacks();

    for stack in stacks.stacks {
        if stack.name.to_lowercase() == r#type.to_lowercase() {
            // populate fields of instance
            instance.version = Some(stack.version);

            for install in stack.trunk_installs {
                let installed_extension = InstalledExtension {
                    name: install.name,
                    version: install.version,
                    created_at: install.created_at,
                };

                instance.installed_extensions.push(installed_extension);
            }

            for extension in stack.extensions {
                let enabled_extension = EnabledExtension {
                    name: extension.name,
                    version: extension.version,
                    created_at: extension.created_at,
                    locations: vec![],
                };

                instance.enabled_extensions.push(enabled_extension);
            }
        }
    }

    config.instances.push(instance);

    let _ = config.write(&Config::full_path(matches));

    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, ArgAction, ArgMatches, Command};

    fn cleanup(matches: &ArgMatches) {
        let path = Config::full_path(matches);
        let mut config = Config::new(matches, &path);

        let _ = &config.instances.pop(); // remove last instance created from test
        let _ = &config.write(&Config::full_path(&matches));
    }

    #[test]
    fn valid_execute_test() {
        // with a valid stack type
        let stack_type = String::from("standard");

        let m = Command::new("create")
            .arg(
                Arg::new("type")
                    .short('t')
                    .long("type")
                    .action(ArgAction::Set)
                    .required(true)
                    .help("The name of a Tembo stack type to create an instance of"),
            )
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .action(ArgAction::Set)
                    .required(true)
                    .help("The name you want to give your instance"),
            )
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .action(ArgAction::Set)
                    .required(true)
                    .help("The port number you want the instance to run on"),
            );

        let matches = &m.get_matches_from(vec![
            "create",
            "-t",
            &stack_type,
            "-n",
            "test_file",
            "-p",
            "5432",
        ]);
        let result = execute(&matches);
        assert_eq!(result.is_ok(), true);

        cleanup(&matches);
    }
}
 */
