use crate::parsing::folderize_title;
use crate::ui::components::{
    add_box_with_label, add_dialog_button, button_secondary, button_standard,
};
use crate::{edit_context_lib::types::StdCommand, ui::message::Message};
use egui::Ui;

#[derive(Default, Clone, Debug)]
pub enum AppModal {
    Create(StdCommand),
    Edit(StdCommand),
    Delete(StdCommand),
    #[default]
    None,
}

impl AppModal {
    pub fn default_create() -> Self {
        AppModal::Create(StdCommand::default())
    }
}

fn command_input_menu(ui: &mut Ui, cmd: &mut StdCommand) {
    add_box_with_label(ui, "Title: ", &mut cmd.title);

    add_box_with_label(ui, "Command: ", &mut cmd.command);

    if let Some(path) = add_dialog_button(
        ui,
        "Run File",
        &[("Executable", &["exe"]), ("All files", &["*"])],
    ) {
        if let Some(str) = path.to_str() {
            cmd.command = format!("\"{}\" %V", str);
        }
    }

    add_box_with_label(ui, "Icon: ", &mut cmd.icon);

    if let Some(path) =
        add_dialog_button(ui, "Choose Icon", &[("Iconable", &["exe", "dll", "ico"])])
    {
        if let Some(str) = path.to_str() {
            cmd.icon = format!("{}", str);
        }
    }
}

pub fn show_create_modal(ui: &mut Ui, current_command: StdCommand) -> Message {
    let mut new_command = current_command;

    command_input_menu(ui, &mut new_command);

    ui.add_space(10f32);

    ui.horizontal(|ui: &mut Ui| {
        if ui.add(button_standard("Confirm")).clicked() {
            let mut command = new_command.clone(); // TODO, Get rid of this clone
            command.folder = folderize_title(&command.title);
            Message::AddCommand(new_command.clone()) // TODO: Get rid of this clone
        } else if ui.add(button_secondary("Cancel")).clicked() {
            Message::UpdateModal(AppModal::None)
        } else {
            Message::UpdateModal(AppModal::Create(new_command))
        }
    })
    .inner
}

pub fn show_edit_modal(ui: &mut Ui, current_command: StdCommand) -> Message {
    let mut new_command = current_command;

    command_input_menu(ui, &mut new_command);

    ui.add_space(10f32);

    ui.horizontal(|ui| {
        if ui.add(button_standard("Confirm")).clicked() {
            Message::UpdateCommand(new_command)
        } else if ui.add(button_secondary("Cancel")).clicked() {
            Message::UpdateModal(AppModal::None)
        } else {
            Message::UpdateModal(AppModal::Edit(new_command))
        }
    })
    .inner
}

pub fn show_delete_modal(ui: &mut Ui, current_command: StdCommand) -> Message {
    ui.heading("Are you sure you want to delete this?");
    ui.add_space(10f32);

    ui.horizontal(|ui| {
        if ui.add(button_standard("Yes")).clicked() {
            Message::RemoveCommand(current_command)
        } else if ui.add(button_standard("No")).clicked() {
            Message::UpdateModal(AppModal::None)
        } else {
            Message::None
        }
    })
    .inner
}
