use eframe::egui::Ui;
use eframe::egui::{Align, Layout, TextEdit};

use crate::config;
use crate::status::Status;


pub fn render_bottom_panel(ui: &mut Ui, status: &mut Status, delete_checked: &mut bool) {
    let mut status_text = status.get_string();

    ui.add_space(5.0);
    ui.add_sized(
        [ui.available_width() - 20.0, 80.0], 
        TextEdit::multiline(&mut status_text)
            .desired_rows(5)
            .interactive(false)
    );

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.checkbox(delete_checked, "delete map after import");

        ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
            ui.add_space(10.0);

            if ui.link("Github").clicked() {
                open::that(config::GITHUB_LINK).unwrap();
                status.insert_status("opened Github link".to_string());
            }
            ui.label(" | ");
            if ui.link("BeatSaver Maps").clicked() {
                open::that(config::BEATSABER_LINK).unwrap();
                status.insert_status("opened BeatSaber link".to_string());
            }
        });
    });
}