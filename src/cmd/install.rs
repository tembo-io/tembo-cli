use crate::{Deserialize, Serialize};
use clap::{ArgMatches, Command};
use spinners::{Spinner, Spinners};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command as ShellCommand;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Stacks {
    stacks: Vec<StackDetails>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct StackDetails {
    name: String,
    description: String,
    stack_version: String,
    trunk_installs: Vec<TrunkInstall>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TrunkInstall {
    name: String,
    version: String, // needs to be parsed as a Version of semver
}

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("install")
        .about("Installs a local Tembo flavored version of Postgres")
        .arg(arg!(-s --stack "A Tembo stack type to install"))
}

pub fn execute(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // if requirements are met (Docker process found)
    match crate::cli::Docker::installed_and_running() {
        Ok(_) => println!("- Docker was found and appears running"),
        Err(e) => eprintln!("{}", e), // TODO: return error
    }

    match build_container() {
        Ok(_) => println!("- Stack was installed"),
        Err(e) => eprintln!("{}", e), // TODO: return error
    }

    match install_stack_config() {
        Ok(_) => println!("- Stack config applied, extensions installed via Trunk"),
        Err(e) => eprintln!("{}", e), // TODO: return error
    }

    // TODO: persist the stack config
    //
    // TODO: inform user of what was installed
    //
    Ok(())
}

fn install_stack_config() -> Result<(), Box<dyn Error>> {
    let file = "./resources/stacks.yaml";
    let mut file = File::open(file).expect("Unable to open stack config file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read stack config file");
    let deserialized_stack: Stacks = serde_yaml::from_str(&contents).unwrap();

    for stack in deserialized_stack.stacks {
        for extension in stack.trunk_installs {
            let _ = install_extension(extension);
        }
    }

    return Ok(());
}

fn install_extension(extension: TrunkInstall) -> Result<(), Box<dyn Error>> {
    let mut sp = Spinner::new(Spinners::Dots12, "Installing extension".into()); // TODO: say what
                                                                                // extension?

    let output = if cfg!(target_os = "windows") {
        ShellCommand::new("cmd")
            .args([
                "/C",
                format!(
                    "docker-compose run bash && trunk install {}",
                    extension.name
                )
                .as_str(),
            ])
            .output()
            .expect("failed to execute process")
    } else {
        ShellCommand::new("sh")
            .arg("-c")
            .arg(
                format!(
                    "cd resources && docker-compose run bash && trunk install {}",
                    extension.name
                )
                .as_str(),
            ) // container is already built
            .output()
            .expect("failed to execute process")
    };

    sp.stop_with_message("- Stack extension installed".into());

    let stderr = String::from_utf8(output.stderr).unwrap();

    if !stderr.is_empty() {
        return Err(Box::new(crate::cli::DockerError::new(
            format!("There was an issue installing the extension: {}", stderr).as_str(),
        )));
    } else {
        return Ok(());
    }
}

// builds the container stored in resources
fn build_container() -> Result<(), Box<dyn Error>> {
    let mut sp = Spinner::new(Spinners::Line, "Installing stack".into());

    let output = if cfg!(target_os = "windows") {
        ShellCommand::new("cmd")
            .args([
                "/C",
                "docker-compose build -f resources/docker-compose.yaml", // TODO: verify this path
            ])
            .output()
            .expect("failed to execute process")
    } else {
        ShellCommand::new("sh")
            .arg("-c")
            //.arg("cd resources && docker-compose build --no-cache --quiet")
            .arg("cd resources") // container is already built
            .output()
            .expect("failed to execute process")
    };

    sp.stop_with_message("- Installing stack complete".into());

    let stderr = String::from_utf8(output.stderr).unwrap();

    if !stderr.is_empty() {
        return Err(Box::new(crate::cli::DockerError::new(
            format!("There was an issue building the container: {}", stderr).as_str(),
        )));
    } else {
        return Ok(());
    }
}
