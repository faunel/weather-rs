use super::common;
use super::common::{ProviderErrors, WeatherData, WeatherProvider};
use crate::services::coordinates;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseCurrent {
    /// Temperature in °C
    pub temp_c: f32,
    /// Feels like temperature in °C
    pub feelslike_c: f32,
    /// Relative humidity in %
    pub humidity: f32,
    /// Wind speed in m/s
    pub wind_kph: f32,
    /// Wind gusts speed in m/s
    pub gust_kph: f32,
    /// Pressure in millimeters of mercury
    pub pressure_in: f32,
    /// Precipitation in mm
    pub precip_mm: f32,
    /// Cloudiness in %
    pub cloud: f32,
    /// Visibility in km
    pub vis_km: f32,
    /// UV index
    pub uv: f32,
    /// Weather description
    pub condition: ResponseContition,
}

#[derive(Deserialize, Debug)]
pub struct ResponseContition {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseLocation {
    /// Location name
    pub name: String,
    /// Location region
    pub region: String,
    /// Location country
    pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseWeather {
    pub location: ResponseLocation,
    pub current: ResponseCurrent,
}

/// get weather from service http://api.weatherapi.com/
pub fn get_weather(
    api_key: &String,
    latitude: f64,
    longitude: f64,
) -> Result<ResponseWeather, ProviderErrors> {
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={},{}",
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
pub struct WeatherApiCom {
    /// API key
    pub api_key: String,
}

/// Implementation for the service https://www.weatherapi.com/
impl WeatherProvider for WeatherApiCom {
    fn get_weather(&self, address: &str) -> Result<WeatherData, Box<dyn common::Error>> {
        let coordinates = coordinates::get_coordinates(address)?;

        let latitude = coordinates.lat;
        let longitude = coordinates.lon;
        let weather = get_weather(&self.api_key, latitude, longitude)?;

        let weather_data = WeatherData {
            location: coordinates.display_name,
            temperature: Some(weather.current.temp_c),
            feelslike: Some(weather.current.feelslike_c),
            humidity: Some(weather.current.humidity),
            /// Convert kilometer/hour to meter/sec
            wind_speed: Some(weather.current.wind_kph * 1000.0 / 3600.0),
            /// Convert kilometer/hour to meter/sec
            gust_speed: Some(weather.current.gust_kph * 1000.0 / 3600.0),
            /// Convert inches of mercury (inchHg) to millimeters of mercury (mmHg)
            pressure: Some(weather.current.pressure_in * 25.4),
            precip: Some(weather.current.precip_mm),
            cloud: Some(weather.current.cloud),
            vis: Some(weather.current.vis_km),
            uv: Some(weather.current.uv),
            dev_point: None,
            description: Some(weather.current.condition.text),
        };
        Ok(weather_data)
    }
}
