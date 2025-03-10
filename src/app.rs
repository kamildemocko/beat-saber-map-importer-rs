use std::fs;

use anyhow::{anyhow, Result};
use eframe::{egui::{self, Align, CentralPanel, Context, DroppedFile, Layout}, App};

use crate::{copier::Copier, status::Status, ui::{render_bottom_panel, render_central_view}};

#[derive(Default)]
pub struct MyApp {
    status: Status,
    delete_checked: bool,
    dropped_files: Vec<DroppedFile>,
    copier: Copier,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            status: Status::new(),
            delete_checked: false,
            dropped_files: Vec::new(),
            copier: Copier::new(),
        }
    }

    fn render_ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                render_central_view(ui);
            });

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                render_bottom_panel(ui, &mut self.status, &mut self.delete_checked);
            });
        });
    }

    fn check_new_files(&mut self, ctx: &Context) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        })
    }

    fn process_dropped_files(&mut self) {
        if self.dropped_files.is_empty() {
            return;
        }

        match self.copier.copy_to_game(&self.dropped_files) {
            Ok(_) => self.handle_successful_import(),
            Err(err) => {
                self.status.insert_status(format!("error: {}", err));
            }
        }

        self.dropped_files = Vec::new();
    }

    fn handle_successful_import(&mut self) {
        for map in &self.dropped_files {
            let map_name_res = self.get_map_name_from_dropped_file(map);
            match map_name_res {
                Ok(map_name) => {
                    if self.delete_checked {
                        if let Err(err) = self.try_delete_map(map) {
                            self.status.insert_status(format!("error: {}", err));
                            return
                        }
                    }

                    self.status.insert_status(
                        format!("map {} imported to game", map_name)
                    );
                }
                Err(err) => {
                    self.status.insert_status(format!("error: {}", err));
                }
            }
        }
    }

    fn try_delete_map(&self, map: &DroppedFile) -> Result<()> {
        if let Some(path) = &map.path {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    fn get_map_name_from_dropped_file(&self, map: &DroppedFile) -> Result<String> {
        let map_path = map.path.as_ref()
            .ok_or_else(|| anyhow!("cannot get path from file"))?;

        let map_name = map_path
            .file_stem()
            .map(|p| p.to_string_lossy().to_string())
            .ok_or_else(|| anyhow!("cannot get name from file"))?;
        
        Ok(map_name)
    }

}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_ui(ctx);
        self.check_new_files(ctx);
        self.process_dropped_files();
    }
}
