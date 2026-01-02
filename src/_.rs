use std::{env, error::Error, io::stdin};

mod addtocontext;
mod addtopath;
mod install;
mod types;

use winreg::enums::HKEY_CLASSES_ROOT;

use crate::addtocontext::{
    add_to_context_window, get_current_context_window, BACKGROUND_SUBKEY, DIRECTORY_SUBKEY,
    FILE_SUBKEY,
};
use crate::addtopath::add_to_path;
use crate::install::install;
use crate::types::{ContextCommandInfo, ToVec};

const INSTALL_FLAG: &str = "-i";
const ADD_PATH_FLAG: &str = "-ap";
const ADD_CONTEXT_FLAG: &str = "-ac";
const GET_CONTEXT_FLAG: &str = "-g";

const DIRECTORY_FLAG: &str = "-d";
const FILE_FLAG: &str = "-f";
const BACKGROUND_FLAG: &str = "-b";

const DEFAULT_ICON: &str = "cmd.exe";

fn main() {
    let mut args = env::args();

    // Consume the application argument
    args.next();

    let flag: String = match args.next() {
        Some(flag) => flag,
        None => {
            panic!("Error: Incorrect number of arguments");
        }
    };

    let result = match (flag.as_str(), args.next()) {
        (INSTALL_FLAG, None) => install(),
        (ADD_PATH_FLAG, Some(input_path)) => add_to_path(input_path),
        (ADD_CONTEXT_FLAG, Some(folder)) => {
            let info_res = get_context_info();

            match info_res {
                Ok(info) => add_to_context_window::<String, String, String, String>(
                    &info,
                    HKEY_CLASSES_ROOT,
                    &folder,
                ),
                Err(err) => Err(err),
            }
        }
        (GET_CONTEXT_FLAG, _) => get_context_window(),
        _ => Err("Argument combination is unsupported".into()),
    };

    match result {
        Err(err) => {
            println!("Error: {}", err);
        }
        _ => {
            println!("Success!")
        }
    }

    //let _ = std::io::stdin().lock().read_line(&mut String::new());
}

fn get_context_window() -> Result<(), Box<dyn Error>> {
    let info = get_context_info()?;

    let current_window_elems =
        get_current_context_window::<String, String, String>(&info, HKEY_CLASSES_ROOT)?;

    current_window_elems
        .iter()
        .for_each(|item| println!("{}", item));

    Ok(())
}

fn get_context_info() -> Result<ContextCommandInfo<String, String, String>, Box<dyn Error>> {
    let input = stdin();

    let mut title = String::new();
    let mut icon = String::new();
    let mut command = String::new();
    let mut path_string = String::new();

    println!("Please enter the command title:");
    input.read_line(&mut title)?;

    println!(
        "Please enter the icon for command [default is {}]:",
        DEFAULT_ICON
    );
    input.read_line(&mut icon)?;

    if icon.trim() == "" {
        icon = String::from("cmd.exe");
    }

    println!("Please enter the command:");
    input.read_line(&mut command)?;

    println!("Please enter where the command should appear [options: -f (FILE), -d (DIRECTORY), -b (BACKGROUND)]:");
    input.read_line(&mut path_string)?;

    let path: Vec<String> = match path_string.trim() {
        DIRECTORY_FLAG => DIRECTORY_SUBKEY.to_vec_generic(),
        FILE_FLAG => FILE_SUBKEY.to_vec_generic(),
        BACKGROUND_FLAG => BACKGROUND_SUBKEY.to_vec_generic(),
        _ => return Err("Invalid flag for command info".into()),
    };

    Ok(ContextCommandInfo {
        title: sanitize(title),
        icon: sanitize(icon),
        command: sanitize(command),
        path: path.into(),
    })
}

fn sanitize(str: String) -> String {
    str.trim().to_string()
}

fn acquire_view() -> Result<String, Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let path = out_dir + "\\VIEW";

    let result = read_to_string(path)?;

    Ok(result)
}
