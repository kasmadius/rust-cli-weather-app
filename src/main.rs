use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;
#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
    api_key: String,
    unit: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}
#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
   speed: f64,
   deg: i32, 
}
#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32
}
#[derive(Serialize, Deserialize, Debug)]
struct Sys {
   r#type: f64,
   id: i32,
   country: String,
   sunrise: i32,
   sunset: i32, 
}

impl Forecast {
    async fn get(city: &str, country_code: &str, api_key: &str, unit: &str ) -> Result<Self, ExitFailure> {
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units={}" ,city, country_code, api_key, unit);
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(resp)
    }
}


#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code, &args.api_key, &args.unit).await?;
    println!("our city: {}, our country code: {}", args.city, args.country_code);
    println!("Temperature: {}, Feels Like: {}, Min Temp: {}, Max Temp: {}, Humidity: {}%, Pressure: {}", response.main.temp, response.main.feels_like, response.main.temp_min, 
                                                                                                         response.main.temp_max, response.main.humidity, response.main.pressure);
    Ok(())
}
