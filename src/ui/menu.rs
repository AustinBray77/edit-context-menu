use egui::{Context, TextureOptions, Ui};

use crate::{
    edit_context_lib::types::{Key, KeyProps, Keys, StdCommand},
    ui::{
        appmodal::AppModal,
        appstyle::AppStyle,
        components::{
            add_box_with_label, button_action, button_dropdown, button_heading, button_standard,
        },
        message::Message,
        state::AppState,
    },
};

fn item_dropdown(ui: &mut Ui, item: &StdCommand) -> Option<AppModal> {
    if item.command.is_empty() {
        if ui.add(button_standard("Add Sub-command")).clicked() {
            println!("Add Sub-command clicked!");
        }
    }

    if ui.add(button_dropdown("Edit")).clicked() {
        Some(AppModal::Edit(item.clone()))
    } else if ui.add(button_dropdown("Remove")).clicked() {
        Some(AppModal::Delete(item.clone()))
    } else {
        None
    }
}

pub fn render_context_menu(
    state: &AppState,
    style: &AppStyle,
    ctx: &Context,
    ui: &mut Ui,
) -> Message {
    let message = match &state.path {
        Some((key, index)) => {
            let extension_input_msg = if key.properties == KeyProps::HasExt {
                let mut new_extension: String = key.path.borrow()[0].to_string();

                add_box_with_label(ui, "Extension: ", &mut new_extension);

                let reload_button = ui.add(button_standard("Reload"));

                if *new_extension.as_str() != *key.path.borrow()[0].clone() {
                    Message::SetPath(Some((key.clone().with_extension(new_extension), *index)))
                } else if reload_button.clicked() {
                    Message::ReloadKey((key.clone(), *index))
                } else {
                    Message::None
                }
            } else {
                Message::None
            };

            ui.add_space(10_f32);

            let blank_items = Vec::new();
            let items = state.items.get(*index).unwrap_or(&blank_items);

            let next_modal_opt = items.iter().fold(None, |acc, (item, img)| {
                let handle = ctx.load_texture("", img.clone(), TextureOptions::default());
                let sized_image = egui::load::SizedTexture::new(
                    handle.id(),
                    egui::vec2(style.icon_size, style.icon_size),
                );

                let response = ui
                    .menu_image_text_button(sized_image, &item.title, |ui| item_dropdown(ui, item));

                response.inner.unwrap_or(acc)
            });

            if ui.add(button_action("+ Add New")).clicked() {
                Message::UpdateModal(AppModal::default_create())
            } else if let Some(next_modal) = next_modal_opt {
                Message::UpdateModal(next_modal)
            } else {
                extension_input_msg
            }
        }
        _ => Message::None,
    };

    message
}

pub fn render_heading(keys: &Keys, path: &Option<(Key, usize)>, ui: &mut Ui) -> Message {
    keys.iter()
        .enumerate()
        .fold(Message::None, |acc, (i, key)| match path {
            Some((_, ind)) if *ind == i => {
                ui.add(button_heading(&key.name, true));
                acc
            }
            _ => {
                if ui.add(button_heading(&key.name, false)).clicked() {
                    Message::SetPath(Some((key.deep_clone(), i)))
                } else {
                    acc
                }
            }
        })
}
