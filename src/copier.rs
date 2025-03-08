use std::{fs::File, io, path::PathBuf};

use regex::Regex;
use anyhow::{anyhow, Ok, Result};


pub struct Copier {
    pub game_path: PathBuf,
}

impl Copier {
    pub fn new() -> Result<Self> {
        let steam_install_path = Copier::get_steam_path()?;
        let steamapps_paths = Copier::get_steamapps_path(steam_install_path)?;
        if steamapps_paths.len() == 0 { 
            return Err(anyhow!("no steamapps paths found"))
        }

        let game_folder = Copier::get_game_path(steamapps_paths)?;

        Ok(Self { game_path: game_folder })
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

        let file = File::open(config_file)?;
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