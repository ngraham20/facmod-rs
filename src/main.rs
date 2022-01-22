mod errors;
mod util;
pub use crate::errors::*;
use clap::{App, Arg};

#[tokio::main]
async fn main() {
    if let Err(ref e) = run().await {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
        
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let matches = App::new("Factorio Mod Installer")
    .about("Download mods for Factorio")
    .version("v0.2.0")
    .author("@ngraham20 (GitHub)")
    .arg(Arg::new("config")
        .short('c')
        .long("config")
        .takes_value(true)
        .value_name("FILE")
        .help("Specifies a config file"))
    .arg(Arg::new("mods_folder")
        .short('m')
        .long("mods_folder")
        .takes_value(true)
        .value_name("FOLDER")
        .help("Specifies the location of the Factorio mods folder")).get_matches();
    // init the loggerls
    env_logger::init();

    let configdata = util::load_config(matches.value_of("config").expect("required"))?;
    let fmods = configdata.mod_list;
    
    let mod_dir = match matches.value_of("mods_folder") {
        Some(path) => String::from(path),
        _ => configdata.mod_dir
    };

    // search for the mods on the factorio mod portal
    let fmoddata = util::search_mods(fmods).await?;

    // download mods
    util::download_mods(fmoddata, &mod_dir, &configdata.username, &configdata.api_token).await?;

    Ok(())
}