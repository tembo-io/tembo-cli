const CONFIG_FILE_NAME: &str = "configuration.toml";

#[allow(dead_code)]
use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command as ShellCommand;
use std::process::Output;

pub struct Docker {}

impl Docker {
    pub fn info() -> Output {
        let output = if cfg!(target_os = "windows") {
            ShellCommand::new("cmd")
                .args(["/C", "docker --info"])
                .output()
                .expect("failed to execute process")
        } else {
            ShellCommand::new("sh")
                .arg("-c")
                .arg("docker info")
                .output()
                .expect("failed to execute process")
        };

        return output;
    }

    pub fn installed_and_running() -> Result<(), Box<dyn Error>> {
        println!("- Checking requirements: [Docker]");

        let output = Self::info();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        // determine if docker is installed
        if stdout.is_empty() && !stderr.is_empty() {
            return Err(Box::new(DockerError::new(
                "Docker is not installed, please visit docker.com to install",
            )));
        } else {
            // determine if docker is running
            if !stdout.is_empty() {
                // look for 'Cannot connect to the Docker daemon'
                let connection_err = stdout.find("Cannot connect to the Docker daemon");

                if let Some(_) = connection_err {
                    return Err(Box::new(DockerError::new(
                        "Docker is not running, please start it and try again",
                    )));
                }
            }
        }

        return Ok(());
    }
}

// Define Docker not installed Error
#[derive(Debug)]
pub struct DockerError {
    details: String,
}

impl DockerError {
    pub fn new(msg: &str) -> DockerError {
        DockerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DockerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DockerError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Config<'a> {
    pub file_name: &'a str,
    pub file_path: PathBuf,
}

impl<'a> Config<'a> {
    pub fn new(args: &ArgMatches) -> Config {
        Config {
            file_name: CONFIG_FILE_NAME, // TODO: support passed in args setting the file_name
            file_path: Self::full_path(args),
        }
    }

    pub fn create_config_dir(dir_path: &str) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(dir_path)?;
        Ok(())
    }

    pub fn create_config_file(path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create_new(&path)?; // don't overwrite existing file at path
        file.write_all(b"[configuration]")?; // TODO: don't write any info here, just create
                                             // the new file

        Ok(())
    }

    fn full_path(args: &ArgMatches) -> PathBuf {
        // if file-path was provided
        if let Some(path) = args.get_one::<PathBuf>("file-path") {
            if path.is_relative() {
                env::current_dir()
                    .expect("Unable to determine the current directory")
                    .join(path)
            } else {
                path.to_path_buf()
            }
        } else {
            // if file-path was not provided, use the home directory as a default
            let home_dir = home::home_dir();

            // if home directory can not be determined, use the current directory
            match home_dir {
                Some(mut path) => {
                    path.push(".config");
                    path.push("tembo");
                    path.push(CONFIG_FILE_NAME);

                    return path;
                }
                None => env::current_dir().expect("Unable to determine the current directory"),
            }
        }
    }

    pub fn init(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut full_path = file_path.clone();
        full_path.pop(); // removes any filename and extension

        match Config::create_config_dir(&full_path.to_string_lossy()) {
            Ok(()) => Config::create_config_file(&file_path.to_string_lossy()),
            Err(e) => {
                println!("Directory can not be created, {}", e);

                return Err(e);
            }
        }
    }

    #[allow(dead_code)]
    pub fn append() -> Result<(), Box<dyn Error>> {
        return Ok(());
    }
}
