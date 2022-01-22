use crate::errors::*;
use clap::{App, Arg, ArgMatches};

pub fn parse_args() -> Result<ArgMatches>{
    let matches = App::new("Factorio Mod Installer")
    .about(clap::crate_description!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
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
        .help("Specifies the location of the Factorio mods folder"))
    .after_help(
"CONFIG FILE OPTIONS:
    These should be in standar YAML format and may include:
    - username:  String -- Your factorio.com username for authenticating downloads.
    - api_token: String -- Your factorio.com 'token'. This can be found in your factorio.com
                            account profile.
    - mod_dir:   String -- The relative or absolute path to your mods folder.
    - mod_list:  List   -- This YAML formatted list should contain the url names of the desired
                            mods. These can be found in both the download link and the mod url.")
        .get_matches();

    Ok(matches)
}