mod config;
mod app;
mod ui;
mod status;

use crate::app::MyApp;
use eframe::{egui::ViewportBuilder, run_native, NativeOptions};

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(config::WINDOW_SIZE)
            .with_resizable(false),
        ..Default::default()
    };
    _ = run_native(
        config::TITLE, 
        options, 
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    );
}
