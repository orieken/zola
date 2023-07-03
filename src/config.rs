use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArtifactoryConfig {
    pub url: String,
    pub token: String,
}

impl ArtifactoryConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut settings = Config::default();
        settings.merge(File::new("settings", FileFormat::Toml))?;
        settings.try_into()
    }
}
