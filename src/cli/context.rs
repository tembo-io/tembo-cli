use serde::Deserialize;
use serde::Serialize;

pub const CONTEXT_DEFAULT_TEXT: &str = "version = \"1.0\"

[[environment]]
name = 'local'
target = 'docker'

[[environment]]
name = 'prod'
target = 'tembo-cloud'
org_id = 'ORG_ID'
";

pub fn tembo_home_dir() -> String {
    let mut tembo_home = home::home_dir().unwrap().as_path().display().to_string();
    tembo_home.push_str("/.tembo");
    tembo_home
}

pub fn tembo_context_file_path() -> String {
    return tembo_home_dir() + &String::from("/context");
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Context {
    pub version: String,
    pub environment: Vec<Environment>,
}

// Config struct holds to data from the `[config]` section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Environment {
    pub name: String,
    pub target: String,
    pub org_id: Option<String>,
    pub set: Option<bool>,
}
