use copy_dir::copy_dir;
use std::env::current_dir;
use std::error::Error;
use std::io::{BufRead, stdin};
use std::path::Path;
use winreg::enums::HKEY_CLASSES_ROOT;

use crate::edit_context_lib::addtocontext::{
    BACKGROUND_SUBKEY, DIRECTORY_SUBKEY, add_to_context_window, convert_subkey,
};
use crate::edit_context_lib::addtopath::{add_to_path, valid_path};
use crate::edit_context_lib::types::{ContextCommandInfo, NormalResult};

const DEFAULT_INSTALL_PATH: &str = "C:\\ATPW";
const ICON: &str = "cmd.exe";
const TITLE: &str = "Add To Path";

pub fn install() -> NormalResult {
    // Get desired path from user
    let install_path = get_install_path()?;

    println!("Installing at \"{}\"", install_path);

    // Copy the current application to the program files
    copy_current_to_path(&install_path)?;

    // Add path to path
    add_to_path(&install_path)?;

    // Install program to context window
    add_command_to_window(&install_path)?;

    Ok(())
}

fn get_install_path() -> Result<String, Box<dyn Error>> {
    println!(
        "Where would you like to install? Hit enter for the default, \"{}\".",
        DEFAULT_INSTALL_PATH
    );

    let mut result = String::new();

    let input = stdin();

    input.lock().read_line(&mut result)?;

    if result.trim().len() == 0 {
        result = DEFAULT_INSTALL_PATH.to_string();
    }

    return Ok(result);
}

fn copy_current_to_path<T: AsRef<Path>>(dir: &T) -> NormalResult {
    let cur_dir = current_dir()?;

    copy_dir(cur_dir, dir)?;

    Ok(())
}

fn add_command_to_window<T: Into<String>>(dir_into: T) -> NormalResult {
    let dir: String = dir_into.into();

    if !valid_path(&dir) {
        return Err("Directory does not exist".into());
    }

    // Opens: HKEY_CLASSES_ROOT\Directory\shell\ATPW\command
    let command = format!("\"{}\\add_to_path_window.exe\" -a \"%V\"", dir);

    add_to_context_window(
        &ContextCommandInfo::new(
            TITLE,
            ICON,
            command.clone(),
            &"APTW",
            convert_subkey(&DIRECTORY_SUBKEY).into(),
        ),
        HKEY_CLASSES_ROOT,
    )?;

    add_to_context_window(
        &ContextCommandInfo::new(
            TITLE,
            ICON,
            command,
            &"APTW",
            convert_subkey(&BACKGROUND_SUBKEY),
        ),
        HKEY_CLASSES_ROOT,
    )?;

    Ok(())
}
