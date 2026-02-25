use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;

// ==================== CLI Interface Design ====================

#[derive(Parser)]
#[command(author, version, about = "A simple Weather CLI", long_about = None)]
struct Cli {
    #[arg(short, long)]
    city: String,
}

// ==================== Data Modeling ====================

#[derive(Deserialize, Debug)]
struct GeocodingResponse {
    results: Option<Vec<Location>>,
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

// ==================== Logic ====================

// Fetch coordinates of the city using the Geocoding API
async fn fetch_coordinates(city: &str) -> Result<Location> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );

    let response = reqwest::get(&url)
        .await
        .context("Failed to connect to the geocoding API")?
        .json::<GeocodingResponse>()
        .await
        .context("Failed to parse geocoding JSON")?;

    response
        .results
        .and_then(|mut r| r.pop())
        .context("City not found. Please check your spelling.")
}

// Get weather for the given coordinates
async fn get_weather(lat: f64, lon: f64) -> Result<CurrentWeather> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        lat, lon
    );

    let response = reqwest::get(&url)
        .await
        .context("Failed to connect to the weather API")?
        .json::<WeatherResponse>()
        .await
        .context("Failed to parse weather JSON")?;

    Ok(response.current_weather)
}

// ==================== Main ====================

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();
    println!("Fetching weather data for {}...", cli.city);

    let location = fetch_coordinates(&cli.city).await?;
    let weather = get_weather(location.latitude, location.longitude).await?;

    println!("\n--- Weather in {} ---", location.name);
    println!("Temperature: {}°C", weather.temperature);
    println!("Wind Speed: {} km/h", weather.windspeed);
    println!("------------------------");

    Ok(())
}