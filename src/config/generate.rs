use crate::config::DEFAULT_CONFIG;
use std::fs;
use std::io::Write;

pub fn generate_config_file() {
    let config_dir = dirs::config_dir().expect("Couldn't get config dir");
    let docu_config_dir = config_dir.join("docu");
    fs::create_dir_all(&docu_config_dir).expect("Couldn't create docu config dir");
    let config_file_path = docu_config_dir.join("config.toml");
    if config_file_path.exists() {
        println!(
            "Config file already exists at: {}",
            config_file_path.display()
        );
        return;
    }

    fs::File::create(&config_file_path)
        .expect("Error creating config file")
        .write_all(DEFAULT_CONFIG.as_bytes())
        .expect("Error writing default config file");
    println!(
        "Successfully generated config file at: {}",
        config_file_path.display()
    );
}
