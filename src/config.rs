use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use anyhow::Context;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Project {
    pub name: String,
    pub compose_path: String,
    pub shell_cmd: Option<String>,
}

impl Project {
    pub fn new(name: String, compose_path: String, shell_cmd: Option<String>) -> Self {
        Self {
            name,
            compose_path,
            shell_cmd,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub projects: Vec<Project>
}

impl Config {
    fn get_path() -> anyhow::Result<PathBuf> {
        let project_dirs = ProjectDirs::from("com", "dmitryvakulenko", "cmux")
            .context("Failed to get project directories")?;
        let config_dir = project_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context("Failed to create config directory")?;
        }
        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::get_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path).context("Failed to read config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::get_path()?;
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(path, content).context("Failed to write config file")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load_save() {
        let mut config = Config::default();
        config.projects.push(Project{
            name: "test_project".to_string(),
            compose_path: "/tmp/test".to_string(),
            shell_cmd: None,
        });
        
        let path = Config::get_path().expect("Should get config path");
        
        // Ensure clean state
        if path.exists() {
            fs::remove_file(&path).expect("Should remove old config file");
        }

        config.save().expect("Should save config");
        assert!(path.exists());
        
        let loaded = Config::load().expect("Should load config");
        assert_eq!(loaded.projects[0].name, "test_project");
        assert_eq!(loaded.projects[0].compose_path, "/tmp/test");
        assert_eq!(loaded.projects[0].shell_cmd, None);

        fs::remove_file(path).expect("Should cleanup test config file");
    }
}