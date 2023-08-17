#[allow(dead_code)]
use std::error::Error;
use std::fmt;
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
                "- Docker is not installed, please visit docker.com to install",
            )));
        } else {
            // determine if docker is running
            if !stdout.is_empty() && !stderr.is_empty() {
                let connection_err = stderr.find("Cannot connect to the Docker daemon");

                if let Some(_) = connection_err {
                    return Err(Box::new(DockerError::new(
                        "- Docker is not running, please start it and try again",
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
