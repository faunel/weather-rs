use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{File, OpenOptions},
    io::prelude::*,
    path::PathBuf,
};
use text_io::read;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
/// Available weather providers
pub enum Providers {
    Weatherapi,
    Openweathermap,
    Accuweather,
    Aerisweather,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Fields from the configuration file
pub struct AppConfig {
    pub default_api: Providers,
    pub weatherapi: ServiceConfig,
    pub openweathermap: ServiceConfig,
    pub accuweather: ServiceConfig,
    pub aerisweather: ServiceConfig,
}

impl AppConfig {
    /// Gets the path to the configuration file
    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Get the path of the currently executing binary
        let mut project_root = env::current_exe()?;

        // Determine the project root directory
        project_root.pop(); // Remove the binary name
        project_root.pop(); // Remove the "release" or "debug" directory
        project_root.pop(); // Remove the "target" directory

        project_root.push("config.json");
        Ok(project_root)
    }

    /// Reads the configuration file
    pub fn read_config_file() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = match Self::get_config_path() {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

        // Check if the file exists
        if !file_path.exists() {
            // Create the file if it doesn't exist
            let mut file = File::create(&file_path)?;
            let default_config = AppConfig {
                default_api: Providers::Weatherapi,
                weatherapi: ServiceConfig {
                    name: String::new(),
                    api_key: String::new(),
                },
                openweathermap: ServiceConfig {
                    name: String::new(),
                    api_key: String::new(),
                },
                accuweather: ServiceConfig {
                    name: String::new(),
                    api_key: String::new(),
                },
                aerisweather: ServiceConfig {
                    name: String::new(),
                    api_key: String::new(),
                },
            };
            let json = serde_json::to_string_pretty(&default_config)?;
            file.write_all(json.as_bytes())?;
        }

        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: AppConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Writes data to the configuration file
    pub fn write_config_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = match Self::get_config_path() {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

        let json = serde_json::to_string_pretty(&self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Helper function to update api key
    fn update_key(service_config: &mut ServiceConfig, service_name: String) {
        print!("Enter the API key (current: {}): ", service_config.api_key);
        let new_api_key: String = read!("{}\n");
        if !new_api_key.is_empty() {
            service_config.api_key = new_api_key;
            service_config.name = service_name;
            println!("API key updated successfully");
        }
    }

    /// Updates the API key
    pub fn update_api_key(
        &mut self,
        provider: &Providers,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match provider {
            Providers::Weatherapi => {
                Self::update_key(&mut self.weatherapi, "WeatherApi".to_string())
            }
            Providers::Openweathermap => {
                Self::update_key(&mut self.openweathermap, "OpenWeatherMap".to_string())
            }
            Providers::Accuweather => {
                Self::update_key(&mut self.accuweather, "AccuWeather".to_string())
            }
            Providers::Aerisweather => {
                Self::update_key(&mut self.aerisweather, "AerisWeather".to_string())
            }
        }
        self.write_config_file()
    }

    /// Sets the default weather provider
    pub fn set_default_provider(
        &mut self,
        provider: &Providers,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.default_api = *provider;
        self.write_config_file()
    }
}
