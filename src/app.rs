use std::error::Error;

use egui::{ColorImage, Context, Id, modal};
use winreg::enums::HKEY_CLASSES_ROOT;

use crate::edit_context_lib::addtocontext::{
    add_to_context_window, get_current_context_window, remove_from_context_window,
};
use crate::edit_context_lib::types::{Key, Keys, StdCommand};
use crate::parsing::{command_with_icon, folderize_title, sanitize_command};
use crate::resources::config::AppConfig;
use crate::ui::appmodal::{AppModal, show_create_modal, show_delete_modal, show_edit_modal};
use crate::ui::appstyle::AppStyle;
use crate::ui::components::button_standard;
use crate::ui::menu::{render_context_menu, render_heading};
use crate::ui::message::Message;
use crate::ui::state::AppState;

pub struct App<'a> {
    pub heading: &'a str,
    keys: Keys,
    state: AppState,
    style: AppStyle,
}

fn load_item(key: &Key) -> Result<Vec<(StdCommand, ColorImage)>, Box<dyn Error>> {
    match get_current_context_window(key.clone_path(), HKEY_CLASSES_ROOT) {
        Ok(items) => Ok(items
            .into_iter()
            .map(|item| sanitize_command(item))
            //.into_iter()
            //.parallel_map(|item| command_with_icon(item))
            .map(|item| command_with_icon(item))
            //.into_iter()
            .collect::<Vec<(StdCommand, ColorImage)>>()),
        Err(err) => Err(format!("Failed to get current context window of {}", err).into()),
    }
}

impl<'a> App<'a> {
    pub fn new(config: AppConfig) -> Self {
        Self {
            heading: config.title,
            keys: config.keys,
            state: AppState::default(),
            style: AppStyle {
                icon_size: config.icon_size,
            },
        }
    }

    pub fn reload_items(&mut self) {
        match self
            .keys
            .iter()
            .map(load_item)
            .collect::<Result<Box<[Vec<(StdCommand, ColorImage)>]>, Box<dyn Error>>>()
        {
            Ok(new_items) => self.state.items = new_items,
            Err(err) => self.alert(err.to_string().as_str()),
        }
    }

    fn update_modal(&mut self, modal: AppModal) {
        self.state.modal = modal;
    }

    fn render_modal(&mut self, ctx: &Context) -> Message {
        match self.state.modal.clone() {
            AppModal::Create(command) => {
                modal::Modal::new(Id::new("Add-Window"))
                    .show(ctx, |ui| show_create_modal(ui, command))
                    .inner
            }
            AppModal::Edit(command) => {
                modal::Modal::new(Id::new("Edit-Window"))
                    .show(ctx, |ui| show_edit_modal(ui, command))
                    .inner
            }
            AppModal::Delete(command) => {
                modal::Modal::new(Id::new("Delete-Window"))
                    .show(ctx, |ui| show_delete_modal(ui, command))
                    .inner
            }
            AppModal::None => Message::None,
        }
    }

    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::UpdateModal(modal) => self.update_modal(modal),
            Message::AddCommand(cmd) => self.add_command(cmd),
            Message::UpdateCommand(cmd) => self.edit_command(cmd),
            Message::RemoveCommand(cmd) => self.remove_command(cmd),
            Message::SetPath(path) => self.set_path(path),
            Message::ReloadKey((path, index)) => self.reload_key(path, index), //Message::LoadBackup(path) => self.load_backup(path),
            Message::None => {}
        }
    }

    fn add_command(&mut self, mut cmd: StdCommand) {
        if let Some((key, p_index)) = &self.state.path {
            cmd.path = key.clone_path();
            cmd.folder = folderize_title(&cmd.title);

            match add_to_context_window(&cmd, HKEY_CLASSES_ROOT) {
                Ok(()) => self.state.items[*p_index].push(command_with_icon(cmd)),
                Err(err) => {
                    self.alert(format!("Add to context error: {}", err));
                }
            }
        } else {
            self.alert(format!("You must open a path to add an item."));
        }

        self.close_modal();
    }

    fn edit_command(&mut self, cmd: StdCommand) {
        if let Some((_, p_index)) = &self.state.path {
            match add_to_context_window(&cmd, HKEY_CLASSES_ROOT) {
                Ok(()) => {
                    match self.state.items[*p_index]
                        .iter()
                        .enumerate()
                        .find(|x| x.1.0 == cmd)
                    {
                        Some((index, _)) => {
                            let cur_item = &self.state.items[*p_index][index];

                            // Only reload the icon if the command has a different icon
                            if cur_item.0.icon != cmd.icon {
                                self.state.items[*p_index][index] = command_with_icon(cmd);
                            } else {
                                self.state.items[*p_index][index].0 = cmd;
                            }
                        }
                        None => {
                            self.alert("Unable to edit item, item not found!");
                        }
                    }
                }
                Err(err) => {
                    self.alert(format!("Add to context error: {}", err));
                }
            }
        } else {
            self.alert(format!("You must open a path to edit an item."));
        }

        self.close_modal();
    }

    fn remove_command(&mut self, cmd: StdCommand) {
        if let Some((_, p_index)) = &self.state.path {
            match remove_from_context_window(&cmd, HKEY_CLASSES_ROOT) {
                Ok(()) => {
                    self.state.items[*p_index] = self.state.items[*p_index]
                        .iter()
                        .filter(|x| x.0 != cmd)
                        .map(|x| x.clone())
                        .collect();
                }
                Err(err) => {
                    self.alert(format!("Error removing item: {}", err));
                }
            }
        } else {
            self.alert("Cannot remove item from empty path");
        }

        self.close_modal();
    }

    fn close_modal(&mut self) {
        self.state.modal = AppModal::None;
    }

    fn alert<T: Into<Box<str>>>(&mut self, str: T) {
        self.state.alerts.push(str.into());
    }

    fn draw_alerts(&mut self, ctx: &Context) {
        self.state.alerts = self
            .state
            .alerts
            .iter()
            .filter_map(|alert| {
                modal::Modal::new(Id::new(alert))
                    .show(ctx, |ui| {
                        ui.heading(alert);

                        if ui.add(button_standard("Confirm")).clicked() {
                            None
                        } else {
                            Some(alert)
                        }
                    })
                    .inner
                    .cloned()
            })
            .collect();
    }

    fn set_path(&mut self, path: Option<(Key, usize)>) {
        self.state.path = path;
    }

    fn reload_key(&mut self, key: Key, index: usize) {
        match load_item(&key) {
            Ok(new_list) => self.state.items[index] = new_list,
            Err(err) => {
                self.alert(err.to_string());
                println!("{:?}", &self.keys[index]);
                self.set_path(Some((self.keys[index].clone(), index)));
            }
        }
    }

    /*fn load_backup<T: AsRef<Path>>(&mut self, path: T) {
        use crate::registry_io::reader::RegReader;

        let reader: RegReader = match RegReader::try_read_file(path) {
            Ok(reader) => reader,
            Err(err) => {
                self.alert(format!("Unable to read file: {}", err));
                return;
            }
        };

        let tr = match Transaction::new() {
            Ok(tr) => tr,
            Err(err) => {
                self.alert(format!("Unable to create transaction: {}", err));
                return;
            }
        };

        match reader.load_all_transacted(&tr) {
            Ok(()) => {}
            Err(err) => {
                self.alert(format!(
                    "Unable to load regasshi values into winreg: {}",
                    err
                ));
                return;
            }
        };

        match tr.commit() {
            Ok(()) => (),
            Err(err) => {
                self.alert(format!("Unable to commit transaction: {}", err));
                ()
            }
        }
    }*/
}

impl<'a> eframe::App for App<'a> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //println!("{:?}", ui.style());

            ui.horizontal_top(|ui| {
                ui.heading(self.heading);

                ui.add_space(50f32);

                /*if let Some(reg_file) =
                    add_dialog_button(ui, "Load Backup", &[("Registry Files", &["reg"])])
                {
                    self.load_backup(reg_file);
                }*/
            });

            self.handle_message(
                ui.horizontal_top(|ui| render_heading(&self.keys, &self.state.path, ui))
                    .inner,
            );

            ui.add_space(10f32);

            self.handle_message(render_context_menu(&self.state, &self.style, ctx, ui));

            let msg = self.render_modal(ctx);

            self.handle_message(msg);

            self.draw_alerts(ctx);
        });
    }
}
