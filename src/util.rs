use crate::errors::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use serde::Serialize;
use std::cmp::min;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use urlencoding::decode;

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

pub async fn download_file<T: Serialize + ?Sized>(
    client: &reqwest::Client,
    url: &str,
    params: &T,
    mod_folder: &str,
) -> Result<()> {
    //reqwest setup
    info!("Submitting GET request to {}", &url);
    let res = client
        .get(url)
        .query(params)
        .send()
        .await
        .chain_err(|| format!("Failed to GET from '{}'", &url))?;

    let mut ospath: PathBuf = [mod_folder].iter().collect();
    let fname = {
        let oname = String::from(
            res.url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin"),
        );
        decode(&oname).unwrap().to_string()
    };

    ospath.push(&fname);

    let total_size = res
        .content_length()
        .chain_err(|| format!("Failed to get content length from '{}'", &url))?;

    // indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    // download chunks
    info!("Downloading file {}", fname);
    let mut file =
        File::create(&ospath).chain_err(|| format!("Failed to create file '{}'", fname))?;
    let mut downloaded = 0u64;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.chain_err(|| format!("Failed to create file '{}'", fname))?;
        file.write(&chunk)
            .chain_err(|| format!("Error while writing to file"))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", fname, mod_folder));
    Ok(())
}

// pub async fn download_file<T: Serialize + ?Sized>(target: String, mut path: PathBuf, client: &reqwest::Client, params: &T) -> Result<()> {
//     debug!("Sending GET request to {}", target);
//     let response = client.get(target).query(params).send().await?;

//     let mut dest = {
//         let fname = response
//             .url()<T: Serialize + ?Sized>
//             .path_segments()
//             .and_then(|segments| segments.last())
//             .and_then(|name| if name.is_empty() { None } else { Some(name) })
//             .unwrap_or("tmp.bin");
//         path.push(fname);
//         info!("Downolading file: {}", path.to_str().unwrap());
//         File::create(path)?
//     };
//     let content =  response.text().await?;
//     copy(&mut content.as_bytes(), &mut dest)?;
//     Ok(())
// }
