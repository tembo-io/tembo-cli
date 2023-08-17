use crate::cli::Config;
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::error::Error;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("init")
        .about("Initializes a local environment or project; generates configuration")
        .arg(arg!(-d --dryrun "Provides an input file to the program"))
        .arg(
            Arg::new("file-path")
                .short('f')
                .long("file-path")
                .value_name("dir")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .action(ArgAction::Append)
                .help(
                    "A path to the directory to add to the configuration \
                    file to, default is $HOME/.config/tembo",
                ),
        )
}

pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let config: Config = crate::cli::Config::new(args);
    let dry_run: bool = args.get_flag("dryrun");
    let file_path: PathBuf = config.file_path; // TODO: property of the config object

    if dry_run {
        println!(
            "- config file would be created at: {}",
            &file_path.to_string_lossy()
        );
    } else {
        println!(
            "- config file will be created at: {}",
            &file_path.to_string_lossy()
        );

        // initialize the required directories and file
        match Config::init(file_path.clone()) {
            Ok(_) => println!("- {} was written", &file_path.to_string_lossy()),
            Err(e) => eprintln!("- {}; exiting", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn execute_test() {
        // assert that dry-run doesn't write the file
        let file_path = "./test/dryrun/test.toml";
        let path = Path::new(file_path);
        let m = Command::new("myapp")
            .arg(
                Arg::new("dryrun")
                    .value_parser(clap::value_parser!(bool))
                    .action(ArgAction::Set)
                    .required(false),
            )
            .arg(
                Arg::new("file-path")
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .action(ArgAction::Set)
                    .required(false),
            );

        let _ = execute(&m.get_matches_from(vec!["myapp", "true", &file_path]));
        assert_eq!(path.exists(), false);
    }

    #[test]
    #[ignore]
    fn get_dir_test() {
        // with a file-path
        let file_path = "/foo";
        let m = Command::new("myapp")
            .arg(
                Arg::new("file-path")
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .action(ArgAction::Set)
                    .required(false),
            )
            .get_matches_from(vec!["myapp", &file_path]);

        assert_eq!(get_dir(&m).to_str(), Some(file_path));

        // without a file-path
        let m = Command::new("myapp")
            .arg(
                Arg::new("file-path")
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .action(ArgAction::Set)
                    .required(false),
            )
            .get_matches_from(vec!["myapp"]);

        assert_eq!(get_dir(&m).to_str(), home::home_dir().unwrap().to_str());
    }

    #[test]
    #[ignore]
    fn create_path_and_write_config_file_test() {
        let mut path: PathBuf = env::current_dir().unwrap();
        path.push("tests");
        path.push(".config");
        path.push("tembo");
        path.push("configuration.toml");

        let write = create_path_and_write_config_file(&path);
        assert_eq!(write.is_ok(), true);

        let overwrite = create_path_and_write_config_file(&path);
        assert_eq!(overwrite.is_err(), true);

        let _file = std::fs::remove_file(&*path.to_string_lossy());
        let _dir = std::fs::remove_dir(&*path.to_string_lossy());
    }

    #[test]
    #[ignore]
    fn create_config_dir_test() {
        let mut path: PathBuf = env::current_dir().unwrap();
        path.push("tests");
        path.push(".config");

        let write = create_config_dir(&path.to_string_lossy());
        assert_eq!(write.is_ok(), true);

        let overwrite = create_config_file(&path.to_string_lossy());
        assert_eq!(overwrite.is_err(), true);

        // clean up
        let _ = std::fs::remove_dir(&*path.to_string_lossy());
    }

    #[test]
    #[ignore]
    fn create_config_file_test() {
        let mut path: PathBuf = env::current_dir().unwrap();
        path.push("tests");
        path.push("configuration.toml");

        let write = create_config_file(&path.to_string_lossy());
        assert_eq!(write.is_ok(), true);

        let overwrite = create_config_file(&path.to_string_lossy());
        assert_eq!(overwrite.is_err(), true);

        // clean up
        let _ = std::fs::remove_file(&*path.to_string_lossy());
    }
}
