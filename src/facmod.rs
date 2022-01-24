use crate::errors::*;
use crate::util;
use log::{info};
use std::path::{PathBuf};
use serde::Deserialize;

#[derive(Deserialize)]
struct FacMod {
    name: String,
    enabled: bool
}

#[derive(Deserialize)]
struct FacModList {
    mods: Vec<FacMod>
}

#[derive(Deserialize)]
pub struct FacModConfig {
    pub username: Option<String>,
    pub api_token: Option<String>,
    pub mod_dir: Option<String>,
    pub mod_list: Option<Vec<String>>
}

impl FacModConfig {
    pub fn new() -> Self {
        FacModConfig {
            username: None,
            api_token: None,
            mod_dir: None,
            mod_list: None,
        }
    }
    pub fn from_yaml(path: &str) -> Result<Self> {
        let canonpath = std::fs::canonicalize(path)?;
        let pathstr = canonpath.to_str().unwrap();
        info!("Loading Config: {}", pathstr);
        let conf = serde_yaml::from_value(util::load_yaml(pathstr)?)?;
        Ok(conf)
    }
}

pub fn get_modlist_from_json(path: &str) -> Result<Vec<String>> {
    let canonpath = std::fs::canonicalize(path)?;
    let pathstr = canonpath.to_str().unwrap();
    info!("Loading json file: {}", pathstr);
    let jsondata = util::load_json(pathstr)?;
    let facmodlist: FacModList = serde_json::from_value(jsondata)?;

    // filter out the "base" mod and all disabled mods
    let enabledlist: Vec<FacMod> = facmodlist.mods.into_iter().filter(|x| x.enabled && x.name != "base").collect();
    Ok(enabledlist.into_iter().map(|x| x.name).collect())
}

pub async fn search_mods(fmods: Vec<String>) -> Result<Vec<serde_json::Value>> {
    let client = reqwest::Client::new();
    let mut jsondata: Vec<serde_json::Value> = Vec::new();
    for fmod in fmods {
        info!("Searching mod: {}", fmod);
        let requesturl = format!("https://mods.factorio.com/api/mods/{}", fmod);
        let res = client.get(requesturl).send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        jsondata.push(res);
        info!("Found mod: {}", fmod);
    }
    Ok(jsondata)
}

pub async fn download_mods(fmods: Vec<serde_json::Value>, mod_folder: &str, username: &str, api_token: &str) -> Result<()> {
    let path: PathBuf = [mod_folder].iter().collect();
    info!("Checking mods folder: {}", path.to_str().unwrap());
    if !path.as_path().exists() {
        error_chain::bail!("Path {} does not exist.", path.to_str().unwrap());
    }
    info!("Downloading mods into {}", path.to_str().unwrap());
    let client = reqwest::Client::new();
    for fmod in fmods {
        info!("Downloading: {}", fmod.get("name").unwrap());
        if let Some(releases) = fmod.get("releases") {
            match releases {
                serde_json::Value::Array(r) => {
                    let release = r.last().unwrap();
                    let download_url: String = release.get("download_url").unwrap().to_string();
                    let len = download_url.len();

                    // the download_url has double quotes ("") surrounding it. The slice grabs the middle bits
                    let request_url = format!("https://mods.factorio.com{}", &download_url[1..len-1]);
                    util::download_file(request_url, path.clone(), &client, &[("username", &username), ("token", &api_token)]).await?;
                },
                _ => {}
            };
        }
    }
    Ok(())
}