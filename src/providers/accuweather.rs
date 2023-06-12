use super::common;
use super::common::{ProviderErrors, WeatherData, WeatherProvider};
use crate::services::coordinates;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metric {
    #[serde(rename = "Metric")]
    pub metric: ValueMetric,
}

#[derive(Debug, Deserialize)]
pub struct ValueMetric {
    #[serde(rename = "Value")]
    pub value: f32,
}

#[derive(Debug, Deserialize)]
pub struct WindData {
    #[serde(rename = "Speed")]
    pub speed: Metric,
}

#[derive(Debug, Deserialize)]
pub struct ResponseWeather {
    #[serde(rename = "WeatherText")]
    pub weather_text: String,
    #[serde(rename = "Temperature")]
    pub temperature: Metric,
    #[serde(rename = "RealFeelTemperature")]
    pub real_feel_temperature: Metric,
    #[serde(rename = "RelativeHumidity")]
    pub relative_humidity: f32,
    #[serde(rename = "DewPoint")]
    pub dew_point: Metric,
    #[serde(rename = "Wind")]
    pub wind: WindData,
    #[serde(rename = "WindGust")]
    pub wind_gust: WindData,
    #[serde(rename = "UVIndex")]
    pub uvindex: f32,
    #[serde(rename = "UVIndexText")]
    pub uvindex_text: String,
    #[serde(rename = "Visibility")]
    pub visibility: Metric,
    #[serde(rename = "CloudCover")]
    pub cloud_cover: f32,
    #[serde(rename = "Pressure")]
    pub pressure: Metric,
}

/// get weather from service http://api.weatherapi.com/
pub fn get_weather(
    api_key: &String,
    latitude: f64,
    longitude: f64,
) -> Result<std::vec::IntoIter<ResponseWeather>, ProviderErrors> {
    let search_url = format!(
        "http://dataservice.accuweather.com/locations/v1/cities/geoposition/search?apikey={}&q={},{}",
        api_key, latitude, longitude
    );

    let client = Client::new();
    let response = client
        .get(search_url)
        .send()
        .map_err(ProviderErrors::Request)?;

    let response = if response.status().is_success() {
        response
    } else {
        return Err(ProviderErrors::Status(response.status()));
    };

    let body = response.text().map_err(ProviderErrors::Request)?;

    // Decode the JSON response
    let search_data: serde_json::Value =
        serde_json::from_str(&body).map_err(ProviderErrors::JSON)?;

    let location_key = if let Some(loc) = search_data.get("Key") {
        loc.as_str().ok_or(ProviderErrors::LocationKeyNotFound)?
    } else {
        return Err(ProviderErrors::LocationKeyNotFound);
    };

    let weather_url = format!(
        "http://dataservice.accuweather.com/currentconditions/v1/{}?apikey={}&language=uk-ua&details=true",
        location_key, api_key
    );

    let response = client
        .get(weather_url)
        .send()
        .map_err(ProviderErrors::Request)?;

    let body = response.text().map_err(ProviderErrors::Request)?;
    //println!("{}", body);

    let weather: Vec<ResponseWeather> =
        serde_json::from_str(&body).map_err(ProviderErrors::JSON)?;

    let weather: std::vec::IntoIter<ResponseWeather> = weather.into_iter();

    Ok(weather)
}

#[derive(Debug)]
pub struct AccuWeather {
    /// API key
    pub api_key: String,
}

/// Implementation for the service https://www.weatherapi.com/
impl WeatherProvider for AccuWeather {
    fn get_weather(&self, address: &str) -> Result<WeatherData, Box<dyn common::Error>> {
        let coordinates = coordinates::get_coordinates(address)?;

        let latitude = coordinates.lat;
        let longitude = coordinates.lon;

        let mut weather = get_weather(&self.api_key, latitude, longitude)?;

        let weather = weather.next().ok_or(ProviderErrors::ErrorGetWeatherData)?;

        let weather_data = WeatherData {
            location: coordinates.display_name,
            temperature: Some(weather.temperature.metric.value),
            feelslike: Some(weather.real_feel_temperature.metric.value),
            humidity: Some(weather.relative_humidity),
            /// Convert kilometer/hour to meter/sec
            wind_speed: Some(weather.wind.speed.metric.value),
            /// Convert kilometer/hour to meter/sec
            gust_speed: Some(weather.wind_gust.speed.metric.value),
            /// Convert hPa (hectopascals) to mmHg (millimeters of mercury)
            pressure: Some(weather.pressure.metric.value * 0.750_063_8),
            precip: None,
            cloud: Some(weather.cloud_cover),
            vis: Some(weather.visibility.metric.value),
            uv: Some(weather.uvindex),
            dev_point: Some(weather.dew_point.metric.value),
            description: Some(weather.weather_text),
        };
        Ok(weather_data)
        //Err(Box::new(ProviderErrors::LocationKeyErrDecode))
    }
}
