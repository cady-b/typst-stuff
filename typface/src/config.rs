use serde::Deserialize;

use std::{
    collections::{BTreeMap, HashMap},
    env,
    path::PathBuf,
};

pub fn get_conf() -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(dir() + "config.toml")?;
    Ok(toml::from_str(&file)?)
}

pub fn cache_file() -> String {
    dir() + "cache"
}

fn dir() -> String {
    env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| env::var("HOME").unwrap_or("~/".to_string()) + "/.config/")
        + "typface/"
}

type Version = String;
pub type Opt = HashMap<String, Opts>;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub default: PathBuf,
    #[serde(default)]
    pub opt: Opt,
    #[serde(default)]
    pub discover: Discover,
}

#[derive(Deserialize, Debug)]
pub struct Opts {
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Discover {
    #[serde(default)]
    pub named: BTreeMap<PathBuf, Version>,
}
