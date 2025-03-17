use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use eframe::{egui::{self, Align, CentralPanel, Context, DroppedFile, Layout, Vec2}, App};

use crate::{copier::Copier, status::Status, ui::{render_bottom_panel, render_central_view}};

#[derive(Default)]
pub struct MyApp {
    status: Status,
    delete_checked: bool,
    dropped_files: Vec<DroppedFile>,
    picked_files: Vec<PathBuf>,
    copier: Copier,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self> {
        Ok(Self {
            status: Status::new(),
            delete_checked: false,
            dropped_files: Vec::new(),
            picked_files: Vec::new(),
            copier: Copier::new()?,
        })
    }

    fn render_ui(&mut self, ctx: &Context) {
        // style
        let mut style = egui::Style::default();
        style.spacing.button_padding = Vec2::new(20.0, 10.0);
        ctx.set_style(style);

        // components
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                render_central_view(ui, &mut self.picked_files);
            });

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                render_bottom_panel(ui, &mut self.status, &mut self.delete_checked, &self.copier.game_path);
            });
        });
    }

    fn check_new_files(&mut self, ctx: &Context) {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
    }

    fn convert_dropped_files(dropped_files: &Vec<DroppedFile>) -> Option<Vec<PathBuf>> {
        dropped_files.iter().map(|map| {
            map.path.clone()
        }).collect()
    }

    fn process_dropped_and_picked_files(&mut self) {
        // collect data
        let mut proc_files: Vec<PathBuf> = Vec::new();

        if !self.dropped_files.is_empty() {
            let converted_dropped_files = match MyApp::convert_dropped_files(&self.dropped_files) {
                Some(val) => val,
                None => {
                    self.status.insert_status("error: cannot get path from dropped files".to_string());
                    return;
                }
            };
            proc_files.extend(converted_dropped_files);
        }
        if !self.picked_files.is_empty() {
            proc_files.extend(self.picked_files.clone());
        }

        // if no data
        if proc_files.is_empty() {
            return;
        }

        // process map files
        for map in proc_files {
            let map_name = match self.get_map_name_from_path(&map) {
                Ok(map_name) => map_name,
                Err(err) => {
                    self.status.insert_status(format!("error: {}", err));
                    continue;
                }
            };

            match self.copier.copy_to_game(&map, &map_name) {
                Ok(_) => {
                    if self.delete_checked {
                        if let Err(err) = self.try_delete_map(&map) {
                            self.status.insert_status(format!("error: {}", err));
                            continue;
                        }
                    }
                    self.status.insert_status(
                        format!("map {} imported to game", map_name)
                    );
                },
                Err(err) => {
                    self.status.insert_status(format!("error: {}", err));
                    continue;
                }
            }
        }

        self.reset_dropped_and_picked_files();
    }

    fn reset_dropped_and_picked_files(&mut self) {
        self.dropped_files = Vec::new();
        self.picked_files = Vec::new();
    }


    fn try_delete_map(&self, map: &PathBuf) -> Result<()> {
        fs::remove_file(map)?;

        Ok(())
    }

    fn get_map_name_from_path(&self, map: &PathBuf) -> Result<String> {
        let map_name = map
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
        self.process_dropped_and_picked_files();
    }
}
