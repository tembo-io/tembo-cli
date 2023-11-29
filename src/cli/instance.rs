// Objects representing a user created local instance of a stack
// (a local container that runs with certain attributes and properties)
use crate::cli::config::Config;
use crate::cli::database::Database;
use crate::cli::docker::DockerError;
use crate::cli::extension::Extension;
use crate::cli::stacks;
use crate::cli::stacks::{Stack, TrunkInstall};
use chrono::prelude::*;
use clap::ArgMatches;
use hyper::header;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use simplelog::*;
use spinners::{Spinner, Spinners};
use std::cmp::PartialEq;
use std::error::Error;
use std::process::Command as ShellCommand;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Instance {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub port: Option<String>, // TODO: persist as an <u16>
    pub version: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub installed_extensions: Vec<InstalledExtension>,
    pub enabled_extensions: Vec<EnabledExtension>,
    pub databases: Vec<Database>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstalledExtension {
    pub name: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnabledExtension {
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

#[derive(Debug)]
pub struct InstanceError {
    pub name: String,
}

impl Instance {
    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        let stack = self.stack();

        self.build();

        for install in &stack.trunk_installs {
            let _ = self.install_extension(install);
        }

        for extension in &stack.extensions {
            let _ = self.enable_extension(extension);
        }

        Ok(())
    }

    // Returns the stack the instance is based on
    // TODO: determine if there is a way to return a vector element in a better way
    fn stack(&self) -> Stack {
        let stacks = stacks::define_stacks();
        let stack_type = self.r#type.clone().unwrap().to_lowercase();

        let stack_details: Vec<_> = stacks
            .stacks
            .into_iter()
            .filter(|s| s.name.to_lowercase() == stack_type)
            .collect();

        let stack = stack_details.first().unwrap();

        Stack {
            name: stack.name.clone(),
            description: stack.description.clone(),
            version: stack.version.clone(),
            trunk_installs: stack.trunk_installs.clone(),
            extensions: stack.extensions.clone(),
        }
    }

    // builds (and starts) a new container
    fn build(&self) {
        let port_option = format!(
            "--publish {}:{}",
            self.port.clone().unwrap(),
            self.port.clone().unwrap(),
        );

        let mut command = String::from("cd tembo ");
        command.push_str("&& docker run -d --name ");
        command.push_str(&self.name.clone().unwrap());
        command.push(' ');
        command.push_str(&port_option);
        command.push_str(" tembo-pg");

        let _ = self.run_command(&command);
    }

    // starts the existing container
    pub fn start(&self) {
        let mut command = String::from("cd tembo ");
        command.push_str("&& docker start ");
        command.push_str(&self.name.clone().unwrap());

        let _ = self.run_command(&command);
    }

    fn run_command(&self, command: &str) -> Result<(), Box<dyn Error>> {
        let mut sp = Spinner::new(Spinners::Line, "Starting instance".into());

        let output = ShellCommand::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");

        let message = format!(
            "- Tembo instance started on {}",
            &self.port.clone().unwrap(),
        );
        sp.stop_with_message(message);

        let stderr = String::from_utf8(output.stderr).unwrap();

        if !stderr.is_empty() {
            return Err(Box::new(DockerError::new(
                format!("There was an issue starting the instance: {}", stderr).as_str(),
            )));
        }

        Ok(())
    }

    pub fn install_extension(&self, extension: &TrunkInstall) -> Result<(), Box<dyn Error>> {
        let mut sp = Spinner::new(Spinners::Dots12, "Installing extension".into());

        let mut command = String::from("cd tembo && docker exec ");
        command.push_str(&self.name.clone().unwrap());
        command.push_str(" sh -c 'trunk install ");
        command.push_str(&extension.name.clone().unwrap());
        command.push('\'');

        let output = ShellCommand::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process");

        sp.stop_with_newline();

        let stderr = String::from_utf8(output.stderr).unwrap();

        if !stderr.is_empty() {
            return Err(Box::new(DockerError::new(
                format!("There was an issue installing the extension: {}", stderr).as_str(),
            )));
        } else {
            let mut msg = String::from("- Stack extension installed: ");
            msg.push_str(&extension.name.clone().unwrap());

            println!("{}", msg);

            Ok(())
        }
    }

    fn enable_extension(&self, extension: &Extension) -> Result<(), Box<dyn Error>> {
        let mut sp = Spinner::new(Spinners::Dots12, "Enabling extension".into());

        let locations = extension
            .locations
            .iter()
            .map(|s| s.database.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        let mut command = String::from("docker exec ");
        command.push_str(&self.name.clone().unwrap());
        command.push_str(" sh -c 'psql -U postgres -c create extension if not exists \"");
        command.push_str(&extension.name.clone().unwrap());
        command.push_str("\" schema ");
        command.push_str(&locations);
        command.push_str(" cascade;'");

        let output = ShellCommand::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process");

        let mut msg = String::from("- Stack extension enabled: ");
        msg.push_str(&extension.name.clone().unwrap());

        sp.stop_with_message(msg);

        let stderr = String::from_utf8(output.stderr).unwrap();

        if !stderr.is_empty() {
            return Err(Box::new(DockerError::new(
                format!("There was an issue enabling the extension: {}", stderr).as_str(),
            )));
        } else {
            Ok(())
        }
    }

    pub fn find(args: &ArgMatches, name: &str) -> Result<Instance, InstanceError> {
        let config = Config::new(args, &Config::full_path(args));

        info!("finding config for instance {}", name);

        for instance in &config.instances {
            let i_name = instance.name.clone().unwrap();

            if i_name.to_lowercase() == name.to_lowercase() {
                let existing = Instance { ..instance.clone() };

                return Ok(existing);
            }
        }

        Err(InstanceError {
            name: name.to_string(),
        })
    }

    pub fn deploy(&self, args: &ArgMatches) -> Result<String, Box<dyn Error>> {
        let name = args
            .try_get_one::<String>("name")
            .clone()
            .unwrap()
            .unwrap()
            .as_str();
        info!("finding config for instance {name}");

        let config = Config::new(args, &Config::full_path(args));
        if config.cloud_account.is_none() || config.jwt.is_none() {
            return Err(Box::new(DockerError::new(
                format!(
                    "You need to run {} before deploying instances",
                    "`auth login`"
                )
                .as_str(),
            )));
        }

        let binding = config.cloud_account.unwrap();
        let org_id = binding.organizations.first().unwrap().replace('\"', "");
        let jwt = config.jwt.unwrap();

        let client = reqwest::blocking::Client::new();
        let request_url = format!("https://api.tembo.io/api/v1/orgs/{org_id}/instances");

        let headers: HeaderMap = {
            vec![(
                header::AUTHORIZATION,
                format!("Bearer {}", jwt).parse().unwrap(),
            )]
            .into_iter()
            .collect()
        };

        let payload = json!({
            "instance_name": name,
            "stack_type": "Standard",
            "cpu": "1",
            "environment": "dev",
            "memory": "1Gi",
            "storage": "10Gi",
            "replicas": 1,
            //"extensions": [],
            //"trunk_installs": [],
            //"app_services": {},
            //"connection_pooler": {},
            //"extra_domains_rw": [],
            //"ip_allow_list": [],
            //"postgres_configs": [],
        });

        let res = client
            .post(request_url)
            .headers(headers)
            .json(&payload)
            .send()?;

        match res.status() {
            StatusCode::ACCEPTED => {
                info!("provisioning: https://cloud.tembo.io/orgs/{org_id}/clusters");

                Ok(String::from("accepted"))
            }
            status_code if status_code.is_client_error() => {
                info!("{}", status_code);
                Err(From::from(format!("Client error: {status_code}")))
            }
            _ => Err(From::from("Client error")),
        }
    }
}
