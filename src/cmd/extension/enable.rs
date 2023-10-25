//  extension enable command
use crate::cli::config::Config;
use crate::cli::extension::{Extension, ExtensionLocation};
use crate::cli::instance::{EnabledExtension, Instance, InstanceError};
use chrono::Utc;
use clap::{Arg, ArgAction, ArgMatches, Command};
use simplelog::*;
use std::error::Error;
use std::io;

// example usage: tembo extension enable -i my_instance
pub fn make_subcommand() -> Command {
    Command::new("enable")
        .about("Command used to enable extensions for instances")
        .arg(
            Arg::new("instance")
                .short('i')
                .long("instance")
                .action(ArgAction::Set)
                .required(true)
                .help("The name of the instance to enable the extension on"),
        )
}

pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let config = Config::new(args, &Config::full_path(args));
    let instance_arg = args.try_get_one::<String>("instance").unwrap();

    println!("What extension would you like to enable? Example: pgmq");
    let mut name_str = String::from("");

    io::stdin()
        .read_line(&mut name_str)
        .expect("Failed to read line");
    let name_str = name_str.trim().to_string().replace('\n', "");

    println!("What version of the extension would you like to enable? Example: 0.28.0");
    let mut version_str = String::from("");

    io::stdin()
        .read_line(&mut version_str)
        .expect("Failed to read line");
    let version_str = version_str.trim().to_string().replace('\n', "");

    println!("What schema would you like to enable {} for?", name_str);
    let mut location_str = String::from("");

    io::stdin()
        .read_line(&mut location_str)
        .expect("Failed to read line");
    let location_str = location_str.trim().to_string().replace('\n', "");

    println!(
        "trying to enable extension '{}' on instance '{}' for '{}'",
        &name_str,
        &instance_arg.unwrap(),
        &location_str
    );

    if config.instances.is_empty() {
        println!("- No instances have been configured");
    } else {
        let _ = match Instance::find(args, instance_arg.unwrap()) {
            Ok(instance) => {
                enable_extension(instance, &name_str, &location_str, &version_str, args)
            }
            Err(e) => Err(Box::new(e)),
        };
    }

    Ok(())
}

fn enable_extension(
    instance: Instance,
    name: &str,
    location: &str,
    version: &str,
    args: &ArgMatches,
) -> Result<(), Box<InstanceError>> {
    // TODO: decide if this should just prompt the user to start the instance first
    instance.start();

    // try enabling extension for location
    let extension_location = ExtensionLocation {
        schema: Some(location.to_string()),
        enabled: String::from("true"),
        version: version.to_string(),
    };

    let extension = Extension {
        name: Some(name.to_string()),
        version: Some(version.to_string()),
        created_at: Some(Utc::now()),
        locations: vec![extension_location],
    };

    match instance.enable_extension(&extension) {
        Ok(()) => {
            info!("extension {} enabled", name);
            let _ = persist_config(args, extension);
        }
        Err(e) => error!("there was an error: {}", e),
    }

    Ok(())
}

fn persist_config(args: &ArgMatches, extension: Extension) -> Result<(), Box<dyn Error>> {
    let mut config = Config::new(args, &Config::full_path(args));
    let target_instance = args.try_get_one::<String>("instance");
    let new_extension = EnabledExtension {
        name: extension.name,
        version: extension.version,
        created_at: extension.created_at,
        locations: extension.locations,
    };

    for instance in config.instances.iter_mut() {
        if instance.name.clone().unwrap().to_lowercase()
            == target_instance.clone().unwrap().unwrap().to_lowercase()
        {
            // TODO: support the case where the extension was already previously enabled for a
            // different location
            instance.enabled_extensions.push(new_extension.clone());
        }
    }

    match &config.write(&Config::full_path(args)) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("there was an error: {}", e);
            Err("there was an error writing the config".into())
        }
    }
}
