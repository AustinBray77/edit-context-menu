use std::{error::Error, path::PathBuf};

use ini::Ini;

pub fn get_resource_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = std::env::current_exe()?;

    let exe_directory = match exe_path.parent() {
        Some(parent) => parent,
        None => {
            return Err("No parent directory for executable found".into());
        }
    };

    Ok(exe_directory.join("resources"))
}

pub fn load_config_ini() -> Result<Ini, Box<dyn Error>> {
    let resource_path = get_resource_path()?;

    let config = Ini::load_from_file(resource_path.join("config.ini"))?;

    Ok(config)
}
