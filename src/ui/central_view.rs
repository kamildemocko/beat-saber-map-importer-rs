use eframe::egui::{Color32, FontId, RichText, Ui};


pub fn render_central_view(ui: &mut Ui) {
    ui.add_space(100.0);
    ui.label(RichText::new("drag&drop the BeatSaber map here!")
        .font(FontId::proportional(32.0))
        .color(Color32::WHITE)
    );
}