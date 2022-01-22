mod errors;
pub use crate::errors::*;

fn main() {
    if let Err(ref e) = run() {
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

fn run() -> Result<()> {
    

    Ok(())
}


async fn search_mods(mods: String) -> Result<()> {
    let client = reqwest::Client::new();
    let requesturl = format!("https://mods.factorio.com/api/mods/{}", "graftorio2-hs");
    let res = client.get("https://mods.factorio.com/api/mods/").send().await?;
    let content = res.text().await?;
    Ok(())
}

// params: o-modlist, o-mods dir, o-config.json (username and api token, modlist, mod_dir etc) 
// search mods with https://mods.factorio.com/api/mods/{} (json response)
// download mods with https://mods.factorio.com/{downloadlink} (highest index is latest)
// clean up the filename if necessary