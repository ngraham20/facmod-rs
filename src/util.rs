use crate::errors::*;
use std::fs::File;
use std::io::BufReader;
use log::{debug, info};
use serde::{Serialize};
use std::io::copy;
use std::path::{PathBuf};

pub fn load_yaml(config: &str) -> Result<serde_yaml::Value> {
    let f = File::open(config)?;
    let reader = BufReader::new(f);
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn load_json(config: &str) -> Result<serde_json::Value> {
    let f = File::open(config)?;
    let reader = BufReader::new(f);
    Ok(serde_json::from_reader(reader)?)
}

pub async fn download_file<T: Serialize + ?Sized>(target: String, mut path: PathBuf, client: &reqwest::Client, params: &T) -> Result<()> {
    debug!("Sending GET request to {}", target);
    let response = client.get(target).query(params).send().await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        path.push(fname);
        info!("Downolading file: {}", path.to_str().unwrap());
        File::create(path)?
    };
    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}