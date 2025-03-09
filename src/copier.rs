use std::{fs, io, path::PathBuf};

use eframe::egui::DroppedFile;
use regex::Regex;
use anyhow::{anyhow, Ok, Result};
use zip::ZipArchive;


#[derive(Default)]
pub struct Copier{}

impl Copier {
    pub fn new() -> Self {
        Self{}
    }

    pub fn copy_to_game(&self, paths: &Vec<DroppedFile>) -> Result<()> {
        let steam_install_path = Copier::get_steam_path()?;
        let steamapps_paths = Copier::get_steamapps_path(steam_install_path)?;
        if steamapps_paths.len() == 0 { 
            return Err(anyhow!("no steamapps paths found"))
        }

        let game_folder = Copier::get_game_path(steamapps_paths)?;

        // TODO check file size and allow only X MB
        
        for map in paths {
            let map_path = map.path.clone().unwrap();
            let map_name = &map_path
                .file_stem()
                .ok_or_else(|| anyhow!("Invalid file name"))?
                .to_string_lossy()
                .to_string();
            let destination_folder = game_folder
                .join("Beat Saber_Data")
                .join("CustomLevels")
                .join(&map_name);

            if destination_folder.exists() {
                return Err(anyhow!(format!("map {} is already exists in game folder", map_name)))
            }

            Copier::extract_files(map_path, destination_folder)?;
        }

        Ok(())
    }

    fn extract_files(zip_file: PathBuf, destination: PathBuf) -> Result<()> {
        fs::create_dir(&destination)?;
        let file = fs::File::open(zip_file)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = destination.join(file.name());
            let mut exoutfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut exoutfile).unwrap();
        }

        Ok(())
    }

    // gets steam install path from registry
    fn get_steam_path() -> Result<PathBuf> {
        let hkey = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
            .open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam")?;

        let install_path: String = hkey.get_value("InstallPath")?;

        Ok(PathBuf::from(install_path))
    }

    // discovers game installation paths and returns existing paths
    fn get_steamapps_path(steam_path: PathBuf) -> Result<Vec<PathBuf>> {
        let config_file = steam_path.join(r"steamapps\libraryfolders.vdf");

        let file = fs::File::open(config_file)?;
        let config_file_content = io::read_to_string(file)?;

        let re = Regex::new(r#""path"\s+"(.*)""#)?;
        let matches: Vec<PathBuf> = re.captures_iter(&config_file_content)
            .map(|c| c.get(1).map_or("", |m| m.as_str()))
            .map(|s| PathBuf::from(s))
            .filter(|p| p.exists())
            .collect();

        Ok(matches)
    }

    fn get_game_path(steamapps_paths: Vec<PathBuf>) -> Result<PathBuf> {
        let mut found_paths: Vec<PathBuf> = Vec::new();

        for path in steamapps_paths {
            let search_folder = path.join(r"steamapps\common\Beat Saber");
            if search_folder.exists() {
                found_paths.push(search_folder);
            }
        }

        match found_paths.len() {
            0 => Err(anyhow!("game folder not found")),
            1 => Ok(found_paths.into_iter().next().unwrap()),
            _ => Err(anyhow!("multiple game folders were found")),
        }
    }
}