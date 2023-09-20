// Extensions are defined in Tembo Stacks and can be custom installed by users
// they are extensions to Postgres

use crate::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    pub name: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub locations: Vec<ExtensionLocation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionLocation {
    pub database: String,
    pub enabled: String,
    pub version: String,
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use clap::{Arg, ArgAction, Command};

    #[test]
    #[ignore]
    fn define_extension_test() {
        // given a stack name that matches
        // let app = Command::new("myapp").arg(
        //     Arg::new("stack")
        //         .value_parser(clap::value_parser!(String))
        //         .action(ArgAction::Set)
        //         .required(false),
        // );

        // let matches = app.get_matches_from(vec!["myapp", "standard"]);

        // assert_eq!(define_stack(&matches).unwrap(), "standard");
    }
}
