#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod app;
mod ui;
mod status;
mod copier;

use crate::app::MyApp;
use crate::ui::get_app_icon;

use eframe::{egui::ViewportBuilder, run_native, NativeOptions};
use anyhow::Result;

fn main() -> Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(config::WINDOW_SIZE)
            .with_resizable(false)
            .with_drag_and_drop(true)
            .with_icon(get_app_icon()),
        ..Default::default()
    };

    _ = run_native(
        config::TITLE, 
        options, 
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)?)))
    );

    Ok(())
}
