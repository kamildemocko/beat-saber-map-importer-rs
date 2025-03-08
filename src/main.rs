mod app;
mod ui;

use crate::app::MyApp;
use eframe::{egui::ViewportBuilder, run_native, NativeOptions};

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([640.0, 360.0])
            .with_resizable(false),
        ..Default::default()
    };
    _ = run_native(
        "BeatSaber map importer", 
        options, 
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    );
}
