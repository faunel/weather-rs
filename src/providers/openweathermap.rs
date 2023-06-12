use super::common;
use super::common::{ProviderErrors, WeatherData, WeatherProvider};
use crate::services::coordinates;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherInfo {
    pub main: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct MainInfo {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct WindInfo {
    pub speed: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct CloudsInfo {
    pub all: f32,
}

#[derive(Deserialize, Debug)]
pub struct ResponseWeather {
    pub weather: Vec<WeatherInfo>,
    pub main: MainInfo,
    pub visibility: f32,
    pub wind: WindInfo,
    pub clouds: CloudsInfo,
    pub name: String,
}

/// get weather from service http://api.weatherapi.com/
pub fn get_weather(
    api_key: &String,
    latitude: f64,
    longitude: f64,
) -> Result<ResponseWeather, ProviderErrors> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?units=metric&appid={}&lat={}&lon={}",
        api_key, latitude, longitude
    );

    let client = Client::new();
    let response = client.get(url).send().map_err(ProviderErrors::Request)?;

    let response = if response.status().is_success() {
        response
    } else {
        return Err(ProviderErrors::Status(response.status()));
    };

    let body = response.text().map_err(ProviderErrors::Request)?;

    let weather: ResponseWeather = serde_json::from_str(&body).map_err(ProviderErrors::JSON)?;

    Ok(weather)
}

#[derive(Debug)]
pub struct OpenWeatherMap {
    /// API key
    pub api_key: String,
}

/// Implementation for the service https://openweathermap.org/
impl WeatherProvider for OpenWeatherMap {
    fn get_weather(&self, address: &str) -> Result<WeatherData, Box<dyn common::Error>> {
        let coordinates = coordinates::get_coordinates(address)?;

        let latitude = coordinates.lat;
        let longitude = coordinates.lon;

        let weather = get_weather(&self.api_key, latitude, longitude)?;

        //println!("{:#?}", weather);

        let weather_data = WeatherData {
            location: coordinates.display_name,
            temperature: Some(weather.main.temp),
            feelslike: Some(weather.main.feels_like),
            humidity: Some(weather.main.humidity),
            wind_speed: Some(weather.wind.speed),
            gust_speed: None,
            /// Convert hPa (hectopascals) to mmHg (millimeters of mercury)
            pressure: Some(weather.main.pressure * 0.750_063_8),
            precip: None,
            cloud: Some(weather.clouds.all),
            vis: Some(weather.visibility),
            uv: None,
            dev_point: None,
            description: Some(weather.weather[0].main.to_string()),
        };
        Ok(weather_data)
        //Err(ProviderErrors::Coordinates(false))
    }
}
