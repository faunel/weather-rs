use crate::providers::common;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Location {
    /// Latitude
    #[serde(deserialize_with = "parse_f64")]
    pub lat: f64,
    /// Longitude
    #[serde(deserialize_with = "parse_f64")]
    pub lon: f64,
    /// Display name
    pub display_name: String,
}

fn parse_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

/// Gets Geo Coordinates by the name of the locality
pub fn get_coordinates(address: &str) -> Result<Location, Box<dyn std::error::Error>> {
    let encoded_address = urlencoding::encode(address);
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json",
        encoded_address
    );

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()?;

    if response.status().is_success() {
        let body = response.text()?;
        let json: Vec<Location> = serde_json::from_str(&body)?;
        if let Some(location) = json.first() {
            let lat = location.lat;
            let lon = location.lon;
            let display_name = location.display_name.clone();
            return Ok(Location {
                lat,
                lon,
                display_name,
            });
        }
    }

    Err(Box::new(common::ProviderErrors::LocationError(
        "Location not found".to_owned(),
    )))
}
