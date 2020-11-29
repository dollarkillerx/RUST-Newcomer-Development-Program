use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::environment::{*, Environment, Environment::*};
use crate::err::{CrateError, CrateError::*};

const CONFIG_FILENAME: &str = "conf/Poem.toml";

pub type Result<T> = std::result::Result<T, CrateError>;

#[derive(Debug)]
pub struct Database {
    pub adapter: String,
    pub db_name: String,
    pub pool: u16,
}

#[derive(Debug)]
pub struct BasicConfig {
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    pub database: Option<Database>,
    pub workers: Option<u16>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl BasicConfig {
    pub fn new(env: Environment) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: Environment) -> Self {
        let default_workers: u16 = (num_cpus::get() * 2) as u16;

        match env {
            Environment::Development => {
                BasicConfig {
                    environment: Development,
                    address: "localhost".to_string(),
                    port: 8000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
                }
            }
            Environment::Staging => {
                BasicConfig {
                    environment: Staging,
                    address: "0.0.0.0".to_string(),
                    port: 8000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
                }
            }
            Environment::Production => {
                BasicConfig {
                    environment: Production,
                    address: "0.0.0.0".to_string(),
                    port: 8000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
                }
            }
        }
    }

    pub fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    fn default_from<P>(env: Environment, path: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let mut config = BasicConfig::default(env);

        let config_file_path = path.as_ref().to_path_buf();
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must be rooted in directory.";
            return Err(CrateError::PathError);
        }

        Ok(config)
    }
}

impl PartialEq for BasicConfig {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
            && self.port == other.port
            && self.workers == other.workers
    }
}

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub struct PoemConfig {
    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,
}

impl PoemConfig {
    pub fn read_config() -> Result<PoemConfig> {
        let file = PoemConfig::find()?;

        let mut handle = File::open(&file).map_err(|_| CrateError::PathError)?;
        let mut contents = String::new();
        handle.read_to_string(&mut contents).map_err(|_| CrateError::IOError)?;

        PoemConfig::parse(contents, &file)
    }

    fn find() -> Result<PathBuf> {
        let cwd = env::current_dir().map_err(|_| CrateError::NotFound)?;
        let mut current = cwd.as_path();

        loop {
            let manifest = current.join(CONFIG_FILENAME);
            if fs::metadata(&manifest).is_ok() {  // 会检测是否存在
                return Ok(manifest);
            }

            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Err(CrateError::NotFound)
    }

    fn get_mut(&mut self, env: Environment) -> &mut BasicConfig {
        match self.config.get_mut(&env) {
            Some(config) => config,
            None => panic!("set(): {:?} config missing.", env),
        }
    }

    pub fn active_default_from(filename: Option<&Path>) -> Result<PoemConfig> {
        let mut default = HashMap::new();
        if let Some(path) = filename {
            default.insert(Development, BasicConfig::default_from(Development, &path)?);
            default.insert(Staging, BasicConfig::default_from(Staging, &path)?);
            default.insert(Production, BasicConfig::default_from(Production, &path)?);
        } else {
            default.insert(Development, BasicConfig::default(Development));
            default.insert(Staging, BasicConfig::default(Staging));
            default.insert(Production, BasicConfig::default(Production));
        }

        let mut config = PoemConfig {
            active_env: Environment::action()?,
            config: default,
        };
        Ok(config)
    }

    pub fn action() -> Result<BasicConfig> {
        Ok(BasicConfig::new(Environment::action()?))
    }

    fn parse<P: AsRef<Path>>(src: String, filename: P) -> Result<PoemConfig> {
        let path = filename.as_ref().to_path_buf();
        let table = match src.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(CrateError::ParseError(src, path, err, Some((1, 1))));
            }
            Err(e) => return Err(CrateError::ParseError(src, path, e.to_string(), e.line_col()))
        };

        let mut config = PoemConfig::active_default_from(Some(filename.as_ref()))?;

        for (entry, value) in table {
            let kv_pairs = match value.as_table() {
                Some(table) => table,
                None => return Err(CrateError::BadType(entry, "a table", value.type_str(), Some(path.clone())))
            };
        }

        Ok(config)
    }
}