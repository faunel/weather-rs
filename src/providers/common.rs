use super::common;
pub use std::error::Error;
use std::fmt;

/// Weather data to display to the user
pub struct WeatherData {
    /// Location
    pub location: String,
    /// Temperature in °C
    pub temperature: Option<f32>,
    /// Feels like temperature in °C
    pub feelslike: Option<f32>,
    /// Relative humidity in %
    pub humidity: Option<f32>,
    /// Wind speed in m/s
    pub wind_speed: Option<f32>,
    /// Wind gusts speed in m/s
    pub gust_speed: Option<f32>,
    /// Pressure in millimeters of mercury
    pub pressure: Option<f32>,
    /// Precipitation in mm
    pub precip: Option<f32>,
    /// Cloudiness in %
    pub cloud: Option<f32>,
    /// Visibility in km
    pub vis: Option<f32>,
    /// UV index
    pub uv: Option<f32>,
    /// Dewpoint
    pub dev_point: Option<f32>,
    /// Weather description
    pub description: Option<String>,
}

#[derive(Debug)]
/// Errors in receiving data from weather providers
pub enum ProviderErrors {
    /// Errors reqwest
    Request(reqwest::Error),
    /// Errors JSON
    JSON(serde_json::Error),
    /// Errors get coordinates
    CoordinatesNotFound,
    /// Response status code
    Status(reqwest::StatusCode),

    LocationKeyNotFound,

    LocationKeyErrDecode,

    ErrorGetWeatherData,

    LocationError(String),
}

impl fmt::Display for ProviderErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProviderErrors::Request(err) => write!(f, "Request error: {}", err),
            ProviderErrors::JSON(err) => write!(f, "JSON error: {}", err),
            ProviderErrors::CoordinatesNotFound => write!(f, "Coordinates not found"),
            ProviderErrors::LocationKeyNotFound => write!(f, "Location key not found"),
            ProviderErrors::LocationKeyErrDecode => write!(f, "Error decoding location key"),
            ProviderErrors::Status(status_code) => {
                write!(f, "Response status code: {}", status_code)
            }
            ProviderErrors::ErrorGetWeatherData => write!(f, "Error get weather data"),
            ProviderErrors::LocationError(err) => write!(f, "Location error: {}", err),
        }
    }
}

impl Error for ProviderErrors {}

/// Obtaining data for each of the providers
pub trait WeatherProvider {
    fn get_weather(&self, address: &str) -> Result<WeatherData, Box<dyn common::Error>>;
}

/// Formatting for beautiful display to the user
impl std::fmt::Debug for WeatherData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "location: {}", &self.location)?;

        if let Some(temperature) = &self.temperature {
            writeln!(f, "temperature: {}", temperature)?;
        }

        if let Some(feelslike) = &self.feelslike {
            writeln!(f, "feelslike: {}", feelslike)?;
        }

        if let Some(humidity) = &self.humidity {
            writeln!(f, "humidity: {}", humidity)?;
        }

        if let Some(wind_speed) = &self.wind_speed {
            writeln!(f, "wind_speed: {:.2}", wind_speed)?;
        }

        if let Some(gust_speed) = &self.gust_speed {
            writeln!(f, "gust_speed: {:.2}", gust_speed)?;
        }

        if let Some(pressure) = &self.pressure {
            writeln!(f, "pressure: {:.0}", pressure)?;
        }

        if let Some(precip) = &self.precip {
            writeln!(f, "precip: {}", precip)?;
        }

        if let Some(cloud) = &self.cloud {
            writeln!(f, "cloud: {}", cloud)?;
        }

        if let Some(vis) = &self.vis {
            writeln!(f, "vis: {}", vis)?;
        }

        if let Some(uv) = &self.uv {
            writeln!(f, "uv: {}", uv)?;
        }

        if let Some(description) = &self.description {
            writeln!(f, "description: {}", description)?;
        }

        Ok(())
    }
}
