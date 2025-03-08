use std::{fs::File, io, path::PathBuf};

use regex::Regex;


pub struct Copier;

impl Copier {
    pub fn new() -> Result<(), io::Error> {
        let steam_install_path = Copier::get_steam_path()?;
        let steamapps_paths = Copier::get_steamapps_path(steam_install_path).unwrap();
        println!("{:?}, len: {}", steamapps_paths, steamapps_paths.len());

        Ok(())
    }

    fn get_steam_path() -> Result<PathBuf, io::Error> {
        let hkey = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
            .open_subkey(r#"SOFTWARE\WOW6432Node\Valve\Steam"#)?;

        let install_path: String = hkey.get_value("InstallPath")?;

        Ok(PathBuf::from(install_path))
    }

    fn get_steamapps_path(steam_path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
        let config_file = steam_path.join(r#"steamapps\libraryfolders.vdf"#);

        let file = File::open(config_file)?;
        let config_file_content = io::read_to_string(file)?;

        let re = Regex::new(r#""path"\s+"(.*)""#).unwrap();
        let matches: Vec<PathBuf> = re.captures_iter(&config_file_content)
            .map(|c| c.get(1).map_or("", |m| m.as_str()))
            .map(|s| PathBuf::from(s))
            .filter(|p| p.exists())
            .collect();

        Ok(matches)
    }
}