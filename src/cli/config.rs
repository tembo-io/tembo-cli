const CONFIG_FILE_NAME: &str = "configuration.toml";

#[allow(dead_code)]
use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

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

        return Ok(());
    }

    pub fn create_config_file(path: &str) -> Result<(), Box<dyn Error>> {
        File::create_new(&path)?; // don't overwrite existing file at path

        return Ok(());
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

                    path
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
    pub fn append(file_path: PathBuf, contents: &str) -> Result<(), Box<dyn Error>> {
        // Open a file with append option
        let mut data_file = OpenOptions::new()
            .append(true)
            .open(file_path)
            .expect("cannot open file");

        // Write to a file
        data_file.write(contents.as_bytes()).expect("write failed");

        return Ok(());
    }
}
