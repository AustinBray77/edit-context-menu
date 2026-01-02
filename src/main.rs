mod app;
mod edit_context_lib;
mod icon;
mod install;
mod parsing;
mod registry_io;
mod resources;
mod ui;

use std::error::Error;
use std::io;
use winreg::enums::HKEY_CLASSES_ROOT;

use log::debug;

use crate::app::App;
use crate::edit_context_lib::backup::backup_paths;
use crate::edit_context_lib::types::Key;
use crate::resources::config::AppConfig;
use crate::resources::resources::load_config_ini;

fn main() {
    env_logger::init();

    match run() {
        Err(err) => {
            println!("Critical error:{}", err);
            let input = io::stdin();
            input.read_line(&mut String::new()).unwrap();
        }
        _ => {}
    }
}

/*fn parse_var_or<'a, T: FromStr, U: Into<T>>(var: &'a str, default: U) -> T {
    match env::var(var) {
        Ok(val) => val.parse::<T>().unwrap_or(default.into()),
        Err(_) => default.into(),
    }
}*/

fn run() -> Result<(), Box<dyn Error>> {
    let config: AppConfig = load_config_ini()?.into();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.width, config.height])
            .with_resizable(true),
        ..Default::default()
    };

    debug!("Running...");

    if config.auto_backup {
        backup_paths(
            config.keys.iter().map(Key::clone_path).collect(), // Clone is cheap (Rc::clone)
            HKEY_CLASSES_ROOT,
        )?;
    }

    let mut app = App::new(config);

    app.reload_items();

    eframe::run_native(
        app.heading,
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}
