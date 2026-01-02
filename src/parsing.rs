use std::env;

use egui::ColorImage;
use log::debug;

use crate::{edit_context_lib::types::StdCommand, icon::get_images_from_exe};

pub fn folderize_title<'a>(s: &'a String) -> String {
    s.replace(" ", "")
}

pub fn sanitize_path(path: String) -> String {
    let result = match path.split_once(',') {
        Some((first, _)) => first,
        None => &path,
    };

    let result = match result.split_once("%USERPROFILE%") {
        Some((_, second)) => match env::home_dir() {
            Some(dir) => match dir.as_os_str().to_str() {
                Some(dir_str) => &(dir_str.to_string() + second),
                None => second,
            },
            None => second,
        },
        None => result,
    };

    result.to_string()
}

pub fn sanitize_command(cmd: StdCommand) -> StdCommand {
    StdCommand::new(
        sanitize_path(cmd.title),
        sanitize_path(cmd.icon),
        cmd.command,
        cmd.folder,
        cmd.path,
    )
}

pub fn command_with_icon(item: StdCommand) -> (StdCommand, ColorImage) {
    let color_image = match get_images_from_exe(&item.icon) {
        Ok(img) => img,
        Err(e) => {
            debug!("Error Loading Image: {}, Loading Default Icon Instead", e);
            get_images_from_exe("C:\\Windows\\System32\\shell32.dll").unwrap_or_default()
        }
    };

    (item, color_image)
}
