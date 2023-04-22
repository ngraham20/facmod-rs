mod errors;
mod util;
mod cli;
mod facmod;

pub use crate::errors::*;
use env_logger::Env;

#[tokio::main]
async fn main() {
    // init the loggerls
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    if let Err(ref e) = run().await {
        log::error!("error: {}", e);

        for e in e.iter().skip(1) {
            log::error!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            log::error!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let matches = cli::parse_args()?;

    // first, load the config file if specified
    let mut configdata = facmod::FacModConfig::new();
    if let Some(config) = matches.value_of("config") {
        configdata = facmod::FacModConfig::from_yaml(config)?;
    }

    // overwrite any config values with existing command arguments
    match (matches.value_of("username"),
           matches.value_of("api_token"),
           matches.value_of("mods_folder")) {
        (Some(facuser), _, _) => configdata.username = Some(String::from(facuser)),
        (_, Some(token), _) => configdata.api_token = Some(String::from(token)),
        (_, _, Some(mod_dir)) => configdata.mod_dir = Some(String::from(mod_dir)),
        _ => {}
    }

    // test all options exist
    let mut confres = 0;
    if let None = configdata.username {
        log::error!("A factorio.com username must be specified in either the config file or as a command argument.");
        confres += 1;
    }
    if let None = configdata.api_token {
        log::error!("A factorio.com token must be specified in either the config file or as a command argument.");
        confres += 1;
    }
    if let None = configdata.mod_dir {
        log::error!("A mods directory must be specified in either the config file or as a command argument.");
        confres += 1;
    }

    // all arguments must exist in either the config file or as a command argument
    if confres > 0 {
        error_chain::bail!("Arguments missing from program");
    }
    let mut modlistfile: String = configdata.mod_dir.clone().unwrap();
    modlistfile.push_str("/mod-list.json");
    let fmods = facmod::get_modlist_from_json(modlistfile.as_str());

    let mod_dir = match matches.value_of("mods_folder") {
        Some(path) => String::from(path),
        _ => configdata.mod_dir.unwrap()
    };

    // search for the mods on the factorio mod portal
    let fmoddata = facmod::search_mods(fmods.unwrap()).await?;

    // download mods
    facmod::download_mods(fmoddata, &mod_dir, &configdata.username.unwrap(), &configdata.api_token.unwrap()).await?;

    Ok(())
}