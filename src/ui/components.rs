use std::path::PathBuf;

use egui::{Button, Color32, RichText, Ui};

pub fn custom_button<'a, T: Into<String>>(
    label: T,
    text_color: Color32,
    fill_color: Color32,
) -> Button<'a> {
    let text = RichText::from(label.into()).color(text_color);

    Button::new(text).fill(fill_color)
}

pub fn button_standard<'a, T: Into<String>>(label: T) -> Button<'a> {
    custom_button(label, Color32::BLACK, Color32::GRAY)
}

pub fn button_action<'a, T: Into<String>>(label: T) -> Button<'a> {
    custom_button(label, Color32::BLACK, Color32::GREEN)
}

pub fn button_secondary<'a, T: Into<String>>(label: T) -> Button<'a> {
    custom_button(label, Color32::BLACK, Color32::DARK_GRAY)
}

pub fn button_dropdown<'a, T: Into<String>>(label: T) -> Button<'a> {
    custom_button(label, Color32::WHITE, Color32::TRANSPARENT)
}

pub fn button_heading<'a, T: Into<String>>(label: T, selected: bool) -> Button<'a> {
    let (text_color, fill_color) = if selected {
        (Color32::WHITE, Color32::TRANSPARENT)
    } else {
        (Color32::BLACK, Color32::GRAY)
    };

    let text = RichText::from(label.into()).heading().color(text_color);

    Button::new(text).fill(fill_color)
}

pub fn add_dialog_button(ui: &mut Ui, label: &str, filters: &[(&str, &[&str])]) -> Option<PathBuf> {
    if ui.add(button_standard(label)).clicked() {
        filters
            .into_iter()
            .fold(rfd::FileDialog::new(), |fd, (name, extensions)| {
                fd.add_filter(*name, *extensions)
            })
            .pick_file()
    } else {
        None
    }
}

pub fn add_box_with_label<T: Into<String>>(ui: &mut Ui, label: T, output: &mut String) {
    let label_id = ui.label(label.into()).id;
    ui.text_edit_singleline(output).labelled_by(label_id);
}
