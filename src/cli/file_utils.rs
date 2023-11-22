use simplelog::*;

use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub struct FileUtils {}

impl FileUtils {
    pub fn create_dir(dir_name: String, dir_path: String) -> Result<(), Box<dyn Error>> {
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

    pub fn create_file(
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
}
