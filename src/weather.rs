use reqwest::Error;
use serde::Deserialize;
use std::env;
use chrono::{Duration, NaiveDate, Utc};

#[derive(Deserialize)]
struct ForecastResponse {
    list: Vec<ForecastItem>,
    city: City,
}

#[derive(Deserialize)]
struct ForecastItem {
    dt: i64,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct City {
    name: String,
}

pub async fn fetch_weather(city: &str) -> Result<(), Error> {
    // Get the API key from environment variables
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY not set");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric",
        city, api_key
    );

    let response: ForecastResponse = reqwest::get(&url).await?.json().await?;

    // Display the city name
    println!("Weather forecast for {}:", response.city.name);

    // Get current date and times for today, tomorrow, and day after
    let now = Utc::now();
    let today = now.date_naive();
    let tomorrow = today + Duration::days(1);
    let day_after = today + Duration::days(2);

    // Filter and display forecasts for each day
    display_forecast(&response.list, today, "Today");
    display_forecast(&response.list, tomorrow, "Tomorrow");
    display_forecast(&response.list, day_after, "Day after tomorrow");

    Ok(())
}

fn display_forecast(forecasts: &[ForecastItem], date: NaiveDate, label: &str) {
    // Find forecast items for the specified date
    let daily_forecasts: Vec<_> = forecasts
        .iter()
        .filter(|item| {
            let forecast_date = chrono::DateTime::<Utc>::from_timestamp(item.dt, 0)
                .expect("Invalid timestamp")
                .date_naive();
            forecast_date == date
        })
        .collect();

    if !daily_forecasts.is_empty() {
        println!("\n{}:", label);

        for forecast in daily_forecasts {
            let forecast_time = chrono::DateTime::<Utc>::from_timestamp(forecast.dt, 0).expect("Invalid timestamp");
            println!(
                "{} - Temp: {} °C, Humidity: {}%, Condition: {}",
                forecast_time.format("%H:%M"),
                forecast.main.temp,
                forecast.main.humidity,
                forecast.weather[0].description
            );
        }
    } else {
        println!("\n{}: No forecast data available", label);
    }
}
