use std::{cell::RefCell, error::Error, ffi::OsStr, fmt::Display, rc::Rc};

use winreg::{RegKey, types::ToRegValue};

use crate::edit_context_lib::types::{
    ContextCommandInfo, KeyPath, NormalResult, StdCommand, StdCommandList,
};

pub const DIRECTORY_SUBKEY: [&str; 2] = ["Directory", "shell"];
pub const BACKGROUND_SUBKEY: [&str; 3] = ["Directory", "Background", "shell"];
pub const FILE_SUBKEY: [&str; 2] = ["*", "shell"];

pub fn convert_subkey(subkey: &[&str]) -> KeyPath {
    Rc::new(RefCell::new(
        subkey
            .iter()
            .map(|x| Into::<Box<str>>::into(*x))
            .collect::<Box<[Box<str>]>>(),
    ))
}

fn validate_command_info<
    U: AsRef<OsStr>,
    T: ToRegValue + Display,
    V: ToRegValue + Display,
    W: ToRegValue + Display,
>(
    command_info: &ContextCommandInfo<T, V, W, U>,
) -> NormalResult {
    if command_info.title.to_string().is_empty()
        || command_info.folder.as_ref().is_empty()
        || command_info.path.borrow().is_empty()
    {
        Err("Command_info arguments are invalid, one or more required arguments are empty".into())
    } else {
        Ok(())
    }
}

pub fn add_to_context_window<
    U: AsRef<OsStr>,
    T: ToRegValue + Display,
    V: ToRegValue + Display,
    W: ToRegValue + Display,
>(
    command_info: &ContextCommandInfo<T, V, W, U>,
    hkey: winreg::HKEY,
) -> NormalResult {
    validate_command_info(command_info)?;

    let root = RegKey::predef(hkey);

    let file: RegKey = command_info
        .path
        .borrow()
        .iter()
        .fold(Ok(root), |cur, next| {
            let next_str = next.as_ref();
            cur?.open_subkey(next_str)
        })?;

    let (file, _) = file.create_subkey(&command_info.folder)?;

    // Set name and icon
    file.set_value("", &command_info.title)?;
    file.set_value("Icon", &command_info.icon)?;

    // Command can be empty for multi-level menu options
    if !command_info.command.to_string().is_empty() {
        let (command_file, _) = file.create_subkey("command")?;
        command_file.set_value("", &command_info.command)?;
    }

    Ok(())
}

pub fn get_current_context_window(
    path: KeyPath,
    hkey: winreg::HKEY,
) -> Result<StdCommandList, Box<dyn Error>> {
    let root = RegKey::predef(hkey);

    let file: RegKey = path.borrow().iter().fold(Ok(root), |cur, next| {
        let next_str: &str = next.as_ref();
        cur?.open_subkey(next_str)
    })?;

    let subnames = file
        .enum_keys()
        .filter_map(|x| match x {
            Ok(v) => Some(v),
            _ => None,
        })
        .map(|name| {
            let key = file.open_subkey(&name)?;

            let title: String = key.get_value("")?;
            let icon: String = key.get_value("icon")?;

            let command_key = key.open_subkey("command")?;

            let command: String = command_key.get_value("")?;

            Ok(ContextCommandInfo::new(
                title,
                icon,
                command,
                name,
                Rc::clone(&path),
            ))
        })
        .filter_map(
            |info_result: Result<StdCommand, Box<dyn Error>>| match info_result {
                Ok(info) => Some(info),
                _ => None,
            },
        )
        .collect::<StdCommandList>();

    Ok(subnames)
}

pub fn remove_from_context_window<
    U: AsRef<OsStr> + Display,
    T: ToRegValue + Display,
    V: ToRegValue + Display,
    W: ToRegValue + Display,
>(
    command_info: &ContextCommandInfo<T, V, W, U>,
    hkey: winreg::HKEY,
) -> NormalResult {
    validate_command_info(command_info)?;

    let root = RegKey::predef(hkey);

    let file: RegKey = command_info
        .path
        .borrow()
        .iter()
        .fold(Ok(root), |cur, next| {
            let next_str = next.as_ref();
            cur?.open_subkey(next_str)
        })?;

    println!("{}", &command_info.folder);

    file.delete_subkey_all(&command_info.folder)?;

    Ok(())
}
