#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod app;
mod ui;
mod status;
mod copier;

use crate::app::MyApp;
use eframe::{egui::ViewportBuilder, run_native, NativeOptions};

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(config::WINDOW_SIZE)
            .with_resizable(false)
            .with_drag_and_drop(true),
        ..Default::default()
    };
    _ = run_native(
        config::TITLE, 
        options, 
        Box::new(|cc| Ok(Box::new(MyApp::new(cc))))
    );
}
