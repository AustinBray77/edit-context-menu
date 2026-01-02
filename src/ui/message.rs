//use std::path::Path;

use crate::{
    edit_context_lib::types::{Key, StdCommand},
    ui::appmodal::AppModal,
};

#[derive(Debug)]
pub enum Message {
    UpdateModal(AppModal),
    SetPath(Option<(Key, usize)>),
    ReloadKey((Key, usize)),
    AddCommand(StdCommand),
    UpdateCommand(StdCommand),
    RemoveCommand(StdCommand),
    //LoadBackup(Box<Path>),
    None,
}
