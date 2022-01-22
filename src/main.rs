mod errors;
mod util;
mod cli;
pub use crate::errors::*;

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
    env_logger::init();


    // find all values
    // username
    // api_token
    // mod_dir
    // mod_list

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