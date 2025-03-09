use std::fs;

use eframe::{egui::{self, Align, CentralPanel, DroppedFile, Layout}, App};

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
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                render_central_view(ui);
            });

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                render_bottom_panel(ui, &mut self.status, &mut self.delete_checked);
            });

            if !self.dropped_files.is_empty() {
                match self.copier.copy_to_game(&self.dropped_files) {
                    Ok(_) => {
                        for map in &self.dropped_files {
                            let map_name = map.path.as_ref().unwrap()
                                .file_stem()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap();

                            if self.delete_checked {
                                for f in &self.dropped_files {
                                    fs::remove_file(f.path.as_ref().unwrap()).unwrap();
                                }
                            }

                            self.status.insert_status(
                                format!("map {} imported to game", map_name)
                            );
                        }
                    },
                    Err(err) => {
                        self.status.insert_status(format!("error: {}", err));
                    }
                };

                self.dropped_files = Vec::new();
            }
        });

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        })
    }
}
