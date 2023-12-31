use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::{default::Default, fs, path::Path};

pub type Deps=BTreeMap<String, Dependency>;
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
pub struct Config {
    pub project: Project,
    //#[serde(skip_serializing_if = "Deps::is_empty")]
    pub dependencies: Deps,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
pub struct Project {
    pub name: Option<String>,
    pub author: Option<String>,
    pub python_version: Option<String>,
    pub project_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
pub struct Dependency {
    pub version: String,
    pub extra: Option<String>,
}

impl Config {
    pub fn new(path: &str, _name: &str, _version: &str) -> Self {
        let path_base = Path::new(path);
        let path_file = path_base.join(Path::new("pacify.toml"));

        if !path_base.exists() {
            fs::create_dir_all(path_base).unwrap();
        }

        if !path_file.exists() {
            Self::save(None, path_file.to_str().unwrap().to_string()).unwrap();
        }

        match Self::load(path_file.to_str().unwrap().to_string()) {
            Ok(config) => config,
            Err(_) => {
                println!("failed to load configuration, using default config");
                Self::default()
            }
        }
    }

    pub fn load(path: String) -> Result<Self, std::io::Error> {
        let path = Path::new(&path);
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents).expect("failed to parse config");
        Ok(config)
    }

    pub fn save(content: Option<Config>, path: String) -> Result<(), std::io::Error> {
        let path = Path::new(&path);
        let contents = match content {
            Some(config) => toml::to_string(&config).unwrap(),
            None => toml::to_string(&Config::default()).unwrap(),
        };
        fs::write(path, contents)?;
        Ok(())
    }
}






