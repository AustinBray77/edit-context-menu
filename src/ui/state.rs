use egui::ColorImage;

use crate::{
    edit_context_lib::types::{Key, StdCommand},
    ui::appmodal::AppModal,
};

#[derive(Default)]
pub struct AppState {
    pub modal: AppModal,
    pub path: Option<(Key, usize)>,
    pub alerts: Vec<Box<str>>,
    pub items: Box<[Vec<(StdCommand, ColorImage)>]>,
    // TODO: Implement LRU Cache for dynamically changing paths (extension searches)
    //pub cache: HashMap<KeyPath, Vec<(StdCommand, ColorImage)>>,
}
