mod errors;
mod util;
mod cli;
pub use crate::errors::*;
use env_logger::Env;

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
    let matches = cli::parse_args()?;
    // init the loggerls
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();


    // find all values
    // username
    // api_token
    // mod_dir

    let mut configdata = util::FacModConfig::new();
    if let Some(config) = matches.value_of("config") {
        configdata = util::load_config(config)?;
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
    if let None = configdata.mod_list {
        log::error!("A mods list must be specified in either the config file or as a command argument.");
        confres += 1;
    }

    if confres > 0 {
        error_chain::bail!("Arguments missing from program");
    }

    let fmods = configdata.mod_list;
    
    let mod_dir = match matches.value_of("mods_folder") {
        Some(path) => String::from(path),
        _ => configdata.mod_dir.unwrap()
    };

    // search for the mods on the factorio mod portal
    let fmoddata = util::search_mods(fmods.unwrap()).await?;

    // download mods
    util::download_mods(fmoddata, &mod_dir, &configdata.username.unwrap(), &configdata.api_token.unwrap()).await?;

    Ok(())
}