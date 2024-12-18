use reqwest;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize)]
struct Location {
	name        : String,
	region      : String,
	country     : String,
}

#[derive(Deserialize)]
struct Current {
	temp_c      : f64,
	condition   : Condition,
}

#[derive(Deserialize)]
struct Condition {
	text: String,
}

#[derive(Deserialize)]
struct WeatherData {
	location: Location,
	current: Current,
}

async fn fetch(city: &str) -> Result<WeatherData, String> {
    dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY is not set");

    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}",
        api_key,
        city.trim()
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let data: WeatherData = response.json().await.map_err(|e| e.to_string())?;
        Ok(data)
    } else {
        Err(format!("Error fetching weather data: {}", response.status()))
    }
}

#[tauri::command(async)]
async fn forecast(city: &str) -> Result<String, String> {
	match fetch(city).await {
		Ok(data) => {
			Ok(format!(
				"{}, {}, {}\n{}\n{}",
				data.location.name,
				data.location.region,
				data.location.country,
				data.current.temp_c,
				data.current.condition.text
			))
		}
		Err(_) => Err("Error fetching weather data".to_string()),
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![forecast])
		.run(tauri::generate_context!())
		.expect("error while running the application");
}
