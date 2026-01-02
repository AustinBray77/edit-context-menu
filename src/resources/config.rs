use std::str::FromStr;

use crate::edit_context_lib::types::{Key, KeyProps, Keys};
use ini::{Ini, Properties};

pub type ConfKey<T> = (&'static str, T);

const APP_TITLE: &'static str = "Edit Context Window";

const WIDTH_KEY: ConfKey<f32> = ("APP_WIDTH", 400_f32);
const HEIGHT_KEY: ConfKey<f32> = ("APP_HEIGHT", 600_f32);
const BACKUP_KEY: ConfKey<bool> = ("AUTO_BACKUP", true);
const ICON_KEY: ConfKey<f32> = ("ICON_SIZE", 25_f32);

pub struct AppConfig {
    pub title: &'static str,
    pub width: f32,
    pub height: f32,
    pub auto_backup: bool,
    pub icon_size: f32,
    pub keys: Keys,
}

fn get_parse_or<T: FromStr + Copy>(props: &Properties, key: ConfKey<T>) -> T {
    props
        .get(key.0)
        .map(str::parse::<T>)
        .unwrap_or(Ok(key.1))
        .unwrap_or(key.1)
}

impl From<Ini> for AppConfig {
    fn from(conf: Ini) -> Self {
        let options = conf.section(Some("Options"));

        let (width, height, auto_backup, icon_size) = match options {
            Some(props) => {
                let width = get_parse_or(props, WIDTH_KEY);

                let height = get_parse_or(props, HEIGHT_KEY);

                let auto_backup = get_parse_or(props, BACKUP_KEY);

                let icon_size = get_parse_or(props, ICON_KEY);

                (width, height, auto_backup, icon_size)
            }
            None => (WIDTH_KEY.1, HEIGHT_KEY.1, BACKUP_KEY.1, ICON_KEY.1),
        };

        let keys_sec = conf.section(Some("RegKeys"));

        let keys: Keys = match keys_sec {
            Some(props) => props
                .iter()
                .map(|(name, path)| Key::new(name).with_path(path))
                .collect::<Keys>(),
            None => Box::new([]),
        };

        let key_props = conf.section(Some("RegProps"));

        // Applies the properties to the keys
        // Using an N * M (where N = num keys >= M = num props) approach as the number of keys
        // and props is quite small and thus the cost of a hash map is probably not worth it
        let keys: Keys = match key_props {
            Some(props) => keys
                .into_iter()
                .map(|key| {
                    if let Some(props) = props.iter().find_map(|(name, key_prop)| {
                        if name == key.name.as_str() {
                            Some(KeyProps::from(key_prop))
                        } else {
                            None
                        }
                    }) {
                        key.with_props(props)
                    } else {
                        key
                    }
                })
                .collect::<Keys>(),
            None => keys,
        };

        AppConfig {
            title: APP_TITLE,
            width,
            height,
            auto_backup,
            icon_size,
            keys,
        }
    }
}
