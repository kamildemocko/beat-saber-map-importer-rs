use std::path::PathBuf;

use eframe::egui::Ui;
use eframe::egui::{Align, Layout, TextEdit};

use crate::config;
use crate::status::Status;

const VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn render_bottom_panel(ui: &mut Ui, status: &mut Status, delete_checked: &mut bool, game_folder: &PathBuf) {
    let mut status_text = status.get_string();

    ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
        ui.add_space(15.0);
        ui.label(format!("v{}", VERSION));
    });

    ui.add_sized(
        [ui.available_width() - 30.0, 80.0], 
        TextEdit::multiline(&mut status_text)
            .desired_rows(5)
            .interactive(false)
    );

    ui.horizontal(|ui| {
        ui.add_space(15.0);
        ui.checkbox(delete_checked, "delete map after import");

        ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
            ui.add_space(15.0);

            if ui.link("Github").clicked() {
                if let Err(err) = open::that(config::GITHUB_LINK) {
                    status.insert_status(format!("failed to open link: {}", err));
                } else {
                    status.insert_status("opened Github link".to_string());
                }
            }
            ui.label(" | ");
            if ui.link("map folder").clicked() {
                if let Err(err) = open::that(game_folder) {
                    // FIX
                    status.insert_status(format!("failed to open folder: {}", err));
                } else {
                    status.insert_status("opened map folder".to_string());
                }
            }
            ui.label(" | ");
            if ui.link("BeatSaver maps").clicked() {
                if let Err(err) = open::that(config::BEATSABER_LINK) {
                    status.insert_status(format!("failed to open link: {}", err));
                } else {
                    status.insert_status("opened BeatSaber link".to_string());
                }
            }
        });
    });
}