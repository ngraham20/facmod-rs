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
        .help("Specifies a config file. Values specified here will be overridden by other command arguments."))
    .arg(Arg::new("mods_folder")
        .short('m')
        .long("mods_folder")
        .takes_value(true)
        .value_name("FOLDER")
        .help("Specifies the location of the Factorio mods folder. Overrides config value, if exists."))
    .arg(Arg::new("mod_list")
        .short('l')
        .long("mod_list")
        .takes_value(true)
        .value_name("FILE")
        .help("Specifies the location of the Factorio mod-list.json file. See additional help for the location of this file."))
    .arg(Arg::new("username")
        .short('u')
        .long("username")
        .takes_value(true)
        .value_name("USERNAME")
        .help("Specifies the Factorio username to use for downloading mods. Overrides config value, if exists."))
    .arg(Arg::new("api_token")
        .short('t')
        .long("api_token")
        .takes_value(true)
        .value_name("TOKEN")
        .help("Specifies the Factorio api token to use for downloading mods. Overrides config value, if exists."))
    .after_help(
"CONFIG FILE OPTIONS:
    These should be in standar YAML format and may include:
    - username:  String --  Your factorio.com username for authenticating downloads.
    - api_token: String --  Your factorio.com 'token'. This can be found in your factorio.com
                            account profile.
    - mod_dir:   String --  The relative or absolute path to your mods folder.
    - mod_list:  List   --  This YAML formatted list should contain the url names of the desired
                            mods. These can be found in both the download link and the mod url.

MOD LIST FILE LOCATIONS:
    The file mod-list.json can be found in the mods folder for the Factorio client.
    - WINDOWS:              C:\\Users\\<user>\\AppData\\roaming\\factorio\\mods\\
    - LINUX:                ~/.factorio/mods/
")
        .get_matches();

    Ok(matches)
}