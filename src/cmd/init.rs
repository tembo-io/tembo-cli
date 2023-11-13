use clap::{ArgMatches, Command};
use simplelog::*;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const CONTEXT_DEFAULT_TEXT: &str = "version = \"1.0\"

[local]
target: docker

[prod]
target: tembo-cloud
org_id: ORG_ID_GOES_HERE
";

fn tembo_home_dir() -> String {
    let mut tembo_home = home::home_dir().unwrap().as_path().display().to_string();
    tembo_home.push_str("/.tembo");
    tembo_home
}

// Create init subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("init")
        .about("Initializes a local environment; generates configuration and pulls Docker image")
}

pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match create_dir("home directory".to_string(), tembo_home_dir()) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    let context_file_path = tembo_home_dir() + &String::from("/context");
    match create_file(
        "context".to_string(),
        context_file_path,
        CONTEXT_DEFAULT_TEXT.to_string(),
    ) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    match create_file(
        "config".to_string(),
        "tembo.toml".to_string(),
        "".to_string(),
    ) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    match create_dir(
        "migrations directory".to_string(),
        "tembo-migrations".to_string(),
    ) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

fn create_dir(dir_name: String, dir_path: String) -> Result<(), Box<dyn Error>> {
    if Path::new(&dir_path).exists() {
        info!("Tembo {} path exists", dir_name);
        return Ok(());
    }

    match fs::create_dir_all(dir_path) {
        Err(why) => panic!("Couldn't create {}: {}", dir_name, why),
        Ok(_) => info!("Tembo {} created", dir_name),
    };

    Ok(())
}

fn create_file(
    file_name: String,
    file_path: String,
    file_content: String,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file_path);
    if path.exists() {
        info!("Tembo {} file exists", file_name);
        return Ok(());
    }
    let display = path.display();
    let mut file: File = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    info!("Tembo {} file created", file_name);

    if let Err(e) = file.write_all(file_content.as_bytes()) {
        panic!("Couldn't write to context file: {}", e);
    }
    Ok(())
}
