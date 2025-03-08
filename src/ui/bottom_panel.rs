use eframe::egui::Ui;
use eframe::egui::{Align, Layout, TextEdit};


pub fn render_bottom_panel(ui: &mut Ui, status: &mut String, delete_checked: &mut bool) {
    ui.add_space(5.0);
    ui.add_sized(
        [ui.available_width() - 20.0, 80.0], 
        TextEdit::multiline(status)
            .desired_rows(5)
            .interactive(false)
    );

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.checkbox(delete_checked, "delete map after import");

        ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
            ui.add_space(10.0);

            if ui.link("Github").clicked() {
                open::that("https://github.com/kamildemocko/beat-saber-song-installer.git").unwrap();
            }
            ui.label(" | ");
            if ui.link("BeatSaver Maps").clicked() {
                open::that("https://beatsaver.com/").unwrap();
            }
        });
    });
}