use clap::{ArgMatches, Command};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};

use crate::cli::{docker::Docker, file_utils::FileUtils};
use tera::Tera;

pub const DOCKERFILE: &str = "FROM quay.io/tembo/tembo-local:latest

# Optional:
# Install any extensions you want with Trunk
{% for key, value in extensions %}
RUN trunk install --version {{value.trunk_project_version}} {{value.trunk_project}}
{% endfor %}

# Optional:
# Specify extra Postgres configurations by copying into this directory
COPY postgres.conf $PGDATA/extra-configs
";

pub const POSTGRES_CONF: &str = "shared_preload_libraries = 'pg_stat_statements,pg_partman_bgw'
pg_partman_bgw.dbname = 'postgres'
pg_partman_bgw.interval = 60
pg_partman_bgw.role = 'postgres'";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TemboConfig {
    pub version: String,
    pub defaults: InstanceSettings,
}

// Config struct holds to data from the `[config]` section.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InstanceSettings {
    pub cpu: String,
    pub memory: String,
    pub storage: String,
    pub replicas: u32,
    pub postgres_configurations: PostgresConfig,
    pub extensions: HashMap<String, Extension>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PostgresConfig {
    pub statement_timeout: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Extension {
    pub enabled: bool,
    pub trunk_project: Option<String>,
    pub trunk_project_version: Option<String>,
}

// Create init subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("apply").about("Applies changes to the context set using the tembo config file")
}

pub fn execute(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match Docker::installed_and_running() {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    let instance_settings: HashMap<String, InstanceSettings>;

    match get_instance_settings() {
        Ok(t) => instance_settings = t,
        Err(e) => {
            return Err(e);
        }
    };

    let rendered_dockerfile: String;

    match get_rendered_dockerfile(instance_settings) {
        Ok(t) => rendered_dockerfile = t,
        Err(e) => {
            return Err(e);
        }
    };

    match FileUtils::create_file(
        "Dockerfile".to_string(),
        "Dockerfile".to_string(),
        rendered_dockerfile,
    ) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    match FileUtils::create_file(
        "postgres.conf".to_string(),
        "postgres.conf".to_string(),
        POSTGRES_CONF.to_string(),
    ) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    match Docker::build_run() {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

pub fn get_instance_settings() -> Result<HashMap<String, InstanceSettings>, Box<dyn Error>> {
    let filename = "tembo.toml";

    let contents = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(e) => {
            panic!("Couldn't read context file {}: {}", filename, e);
        }
    };

    let instance_settings: HashMap<String, InstanceSettings> = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            panic!("Unable to load data. Error: `{}`", e);
        }
    };

    Ok(instance_settings)
}

pub fn get_rendered_dockerfile(instance_settings: HashMap<String, InstanceSettings>) -> Result<(String), Box<dyn Error>> {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let _ = tera.add_raw_template("dockerfile", &DOCKERFILE.to_string());
    let mut context = tera::Context::new();
    for (_key, value) in instance_settings.iter() {
        context.insert("extensions", &value.extensions);
    }
    let rendered_dockerfile = tera.render("dockerfile", &context).unwrap();

    Ok(rendered_dockerfile)
}

#[cfg(test)]
mod tests {
    use super::get_instance_settings;

    // NOTE: wrap tests that require a setup and cleanup step
    #[test]
    fn config_tests() {
        get_instance_settings();
    }
}
