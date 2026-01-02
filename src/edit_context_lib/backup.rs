use chrono::Local;
use log::error;
use std::{error::Error, rc::Rc};
use winreg::{HKEY, RegKey, enums::HKEY_CLASSES_ROOT};

use super::get_resource_path;
use crate::edit_context_lib::types::{KeyPath, NormalResult};
use crate::registry_io::writer::RegWriter;

/*fn reg_hexify<'a>(byte: &'a u8) -> String {
    format!("{:02X}", byte).to_lowercase()
}*/

fn hkey_to_string(hkey: HKEY) -> String {
    match hkey {
        HKEY_CLASSES_ROOT => "HKEY_CLASSES_ROOT".to_string(),
        _ => String::new(),
    }
}

fn current_date_time() -> String {
    // Get the current local date and time
    let now = Local::now();
    // Format it as YYYY-MM-DD-hh-mm-ss
    now.format("%Y-%m-%d-%H-%M-%S").to_string()
}

pub fn backup_paths(paths: Rc<[KeyPath]>, hkey: HKEY) -> NormalResult {
    let writer: RegWriter = paths.iter().fold(
        Ok(RegWriter::new()),
        |writer: Result<RegWriter, Box<dyn Error>>, path: &KeyPath| {
            let regkey = path.borrow().iter().fold(
                Ok(RegKey::predef(hkey)),
                |acc: Result<RegKey, Box<dyn Error>>, key: &Box<str>| {
                    Ok(acc?.open_subkey(key.as_ref())?)
                },
            )?;

            let path_name = hkey_to_string(hkey) + "\\" + &path.borrow().join("\\");

            writer?.with_all_subkeys(regkey, path_name)
        },
    )?;

    let resource_path = match get_resource_path() {
        Ok(path) => path,
        Err(e) => {
            error!("Unable to backup paths: {}", e);
            return Ok(());
        }
    };

    let file_path = resource_path.join(format!("backup-{}.reg", current_date_time()));

    writer.write_to(file_path)?;

    Ok(())
}
//fn regashii_to_winreg(value: Value) -> Option<Value> {}

/*
fn extract_path(path: KeyPath, hkey: HKEY) -> Result<Registry, Box<dyn Error>> {
    /*let root = RegKey::predef(hkey);

    let dir: RegKey = path.iter().fold(Ok(root), |cur, next| {
        let next_str: &str = next.as_ref();

        cur?.open_subkey(next_str)
    })?;*/

    //let content = save_all_in_key(dir, path_str)?;

    Ok(Registry::new(Regedit5).with(path_str, Key::new()))
}

fn sanitize_value(val: &str) -> Box<str> {
    let mut result = Vec::new();

    result.reserve(val.len());

    for c in val.chars() {
        if c == '"' {
            result.push('\\');
        }

        result.push(c);
    }

    result.iter().collect()
}

fn save_all_in_key(regkey: RegKey, path: String) -> Result<String, Box<dyn Error>> {
    use winreg::enums::RegType::{REG_DWORD, REG_SZ};

    let heading = format!("\n[{}]\n", &path);

    let result: String = regkey
        .enum_values()
        .filter_map(|x| if let Ok(item) = x { Some(item) } else { None })
        .fold(heading, |mut acc, (key, value)| {
            if key.is_empty() {
                acc.push('@');
            } else {
                acc.push('"');
                acc.push_str(&sanitize_value(&key));
                acc.push('"');
            }
            acc.push('=');

            match &value.vtype {
                REG_SZ => {
                    acc.push('"');
                    acc.push_str(&sanitize_value(&value.to_string()));
                    acc.push('"');
                }
                REG_DWORD => {
                    acc.push_str("dword:");
                    value.bytes.iter().rev().for_each(|byte| {
                        // Windows reg hex is in lowercase
                        acc.push_str(&reg_hexify(byte));
                    });
                }
                REG_EXPAND_SZ => {
                    acc.push_str("hex(2):");
                    value.bytes.iter().for_each(|byte| {
                        // Windows reg hex is in lowercase
                        acc.push_str(&reg_hexify(byte));
                        acc.push_str(",");
                    });
                    acc.pop();
                }
                _ => acc.push_str("\"\""),
            }

            acc.push('\n');
            acc
        });

    let result = regkey
        .enum_keys()
        .filter_map(|x| {
            if let Ok(key) = x {
                if key.is_empty() { None } else { Some(key) }
            } else {
                None
            }
        })
        .fold(Ok(result), |acc: Result<String, Box<dyn Error>>, next| {
            let mut inner_acc = acc?;
            let new_key = regkey.open_subkey(&next)?;
            let new_path = path.clone() + "\\" + &next;

            inner_acc.push_str(&save_all_in_key(new_key, new_path)?);

            Ok(inner_acc)
        })?;

    Ok(result)
}*/
