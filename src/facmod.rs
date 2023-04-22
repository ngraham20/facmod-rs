use crate::errors::*;
use crate::util;
use log::{info};
use std::path::{PathBuf};
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    download_url: String,
    // file_name: String,
    // info_json: serde_json::Value
}

#[derive(Deserialize)]
pub struct FacModPortalResult {
    // downloads_count: usize,
    name: String,
    // owner: String,
    releases: Vec<Release>,
    // summary: String,
    // title: String,
    // category: String,
    // score: f64,
    // thumbnail: String,
}

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
}

impl FacModConfig {
    pub fn new() -> Self {
        FacModConfig {
            username: None,
            api_token: None,
            mod_dir: None,
        }
    }
    pub fn from_yaml(path: &str) -> Result<Self> {
        let canonpath = std::fs::canonicalize(path)?;
        let pathstr = canonpath.to_str().unwrap();
        info!("Loading Config: {}", pathstr);
        let conf:FacModConfig = serde_yaml::from_value(util::load_yaml(pathstr)?)?;
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

pub async fn search_mods(fmods: Vec<String>) -> Result<Vec<FacModPortalResult>> {
    let client = reqwest::Client::new();
    let mut modresults: Vec<FacModPortalResult> = Vec::new();
    for fmod in fmods {
        info!("Searching mod: {}", fmod);
        let requesturl = format!("https://mods.factorio.com/api/mods/{}", fmod);
        let res = client.get(requesturl).send()
            .await?
            .json::<FacModPortalResult>()
            .await?;
        modresults.push(res);
        info!("Found mod: {}", fmod);
    }
    Ok(modresults)
}

pub async fn download_mods(fmods: Vec<FacModPortalResult>, mod_folder: &str, username: &str, api_token: &str) -> Result<()> {
    let path: PathBuf = [mod_folder].iter().collect();
    let pathstr = path.to_str().chain_err(|| format!("Failed to create string from path."))?;
    info!("Checking mods folder: {}", pathstr);
    if !path.as_path().exists() {
        error_chain::bail!("Path {} does not exist.", pathstr);
    }
    info!("Downloading mods into {}", pathstr);
    let client = reqwest::Client::new();
    for fmod in fmods {
        info!("Downloading: {}", fmod.name);
        let latest = fmod.releases.last().chain_err(|| format!("There are no releases for this mod."))?;
        let request_url = format!("https://mods.factorio.com{}", &latest.download_url);
        util::download_file(&client, &request_url, &[("username", &username), ("token", &api_token)], mod_folder).await?;
    }
    Ok(())
}