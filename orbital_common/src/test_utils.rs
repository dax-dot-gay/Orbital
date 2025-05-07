#![allow(dead_code)]
use std::path::{Path, PathBuf};

use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub(crate) struct TestConfigurationSteamExpect {
    #[serde(default)]
    pub game_folder: PathBuf,
    #[serde(default)]
    pub community_folder: PathBuf,
    #[serde(default)]
    pub docs_folder: PathBuf,
    #[serde(default)]
    pub paks_folder: PathBuf,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub(crate) struct TestConfigurationSteam {
    #[serde(default)]
    pub steam_library: PathBuf,

    #[serde(default)]
    pub expect: TestConfigurationSteamExpect,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub(crate) struct TestConfiguration {
    #[serde(default)]
    pub steam: TestConfigurationSteam,
}

impl TestConfiguration {
    pub(crate) fn new() -> Self {
        let config_path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests.toml");
        if !config_path.exists() {
            return Self::default();
        }

        let result = Figment::new()
            .merge(Toml::file(config_path))
            .extract()
            .unwrap_or_default();

        result
    }
}
