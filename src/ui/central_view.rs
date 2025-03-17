use std::path::PathBuf;

use eframe::egui::{Color32, FontId, RichText, Ui};


pub fn render_central_view(ui: &mut Ui, df: &mut Vec<PathBuf>) {
    ui.add_space(80.0);
    ui.label(RichText::new("drag&drop the BeatSaber map here!")
        .font(FontId::proportional(32.0))
        .color(Color32::WHITE)
    );
    ui.add_space(20.0);
    if ui.button("or choose a file..")
        .on_hover_text("pick a map...")
        .clicked() {
            if let Some(files) = rfd::FileDialog::new()
                .add_filter("zipped map file", &["zip"])
                .pick_files() {
                df.extend(files);
            }
        }
}