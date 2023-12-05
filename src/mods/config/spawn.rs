use std::path::Path;

use crate::mods::types::Config;

pub async fn spawn_config<T: AsRef<Path>>(path: &T) -> Result<Config, String> {
    let raw_config = Config::new_from_path(path).await;
    let config: Config = match raw_config {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err("failed to open config".to_string());
        }
    };
    // check unique name
    let mut names = Vec::new();
    for site in config.sites_config.iter() {
        if names.contains(&site.name) {
            eprintln!("Error: duplicate site name");
            return Err("duplicate site name".to_string());
        }
        names.push(site.name.clone());
    }
    Ok(config)
}

pub async fn init_config<T: AsRef<Path>>(path: &T) -> Result<Config, String> {
    let config = match spawn_config(path).await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err("failed to init config".to_string());
        }
    };
    crate::mods::statics::CONFIG.lock().await.sites_config = config.sites_config.clone();
    Ok(config)
}
