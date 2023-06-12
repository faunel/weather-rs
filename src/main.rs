pub mod providers;
pub mod services;
use clap::Parser;
use providers::{accuweather, common, openweathermap, weatherapi};
use services::{cli, config};

fn main() {
    let mut config = config::AppConfig::read_config_file().unwrap();

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Conf { provider } => {
            if let Err(err) = config.update_api_key(provider) {
                println!("UpdateApiKey Error: {}", err);
            }
        }
        cli::Commands::Get { address } => show_weather(address, &config),
        cli::Commands::Default { provider } => {
            if let Err(err) = config.set_default_provider(provider) {
                println!("SetDefaultProvider Error: {}", err);
            }
        }
    }
}

fn show_weather(address: &str, config: &config::AppConfig) {
    let weather_provider: (Box<dyn common::WeatherProvider>, String) = match config.default_api {
        config::Providers::Weatherapi => {
            let bind = weatherapi::WeatherApiCom {
                api_key: config.weatherapi.api_key.to_string(),
            };
            (Box::new(bind), config.weatherapi.name.to_string())
        }
        config::Providers::Openweathermap => {
            let bind = openweathermap::OpenWeatherMap {
                api_key: config.openweathermap.api_key.to_string(),
            };
            (Box::new(bind), config.openweathermap.name.to_string())
        }
        config::Providers::Accuweather => {
            let bind = accuweather::AccuWeather {
                api_key: config.accuweather.api_key.to_string(),
            };
            (Box::new(bind), config.accuweather.name.to_string())
        }
        config::Providers::Aerisweather => todo!(),
    };

    println!("Provider: {}\n", weather_provider.1);

    match weather_provider.0.get_weather(address) {
        Ok(response) => println!("{:#?}", response),
        Err(err) => eprintln!("{}", err),
    }
}
