## Installation
```bash
## from Github
$ cargo install --git https://github.com/vodrazka/facmod-rs

## manually from source
$ cargo install --path <path_to_repo>
```

## Config YAML
Instead of specifying each parameter manually as a command argument, you may instead use a config file (yaml), as shown below. Any command arguments specified along side `-c <CONFIG>` will override those specified in the config file. This allows the config file to be used as defaults, while still retaining the ability to make manual changes.

```yaml
username: "default"
api_token: "default"
mod_dir: "/opt/factorio/server/mods/"
```

### username
This is your Factorio username.

### api_token
You can find your token at https://factorio.com/profile. Just click **reveal** to see it. While this is safer to use than a password, it's still sensitive information, so make sure to lock down user privileges to the config file so this isn't leaked.

### mod_dir
This is the path to the mods directory. Using the absolute path is best, but relative paths do work.

### mod_list
This should be using the **url version** of the mod name. For example, if downloading https://mods.factorio.com/mod/space-exploration, then the name would be `space-exploration`.
