use chrono::format;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

// http://api.openweathermap.org/data/2.5/weather?q=Beijing&APPID=c6c7ff2ebd36ff0ba4bbb220e1c4b948&units=metric

#[derive(StructOpt, Debug)]
struct Input {
    city: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temperature {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    feels_like: f64,
    humidity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    sunrise: i32,
    sunset: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Weather {
    main: Temperature,
    sys: Sys,
    coord: Coord,
}

impl Weather {
    async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&APPID=c6c7ff2ebd36ff0ba4bbb220e1c4b948&units=metric
      ", city);
        let url = Url::parse(url.as_str())?;
        let response = reqwest::get(url).await?.json::<Weather>().await?;

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    let response = Weather::get(&input.city).await.unwrap();
    println!(
        " 当前温度：{}℃ \n 今日最低温：{}℃ \n 今日最高温：{}℃ \n 体感温度：{}℃ \n 湿度：{}% \n 日出时间：{} \n 日落时间：{} \n 当前经度：{} \n 当前纬度：{}",
        response.main.temp,
        response.main.temp_min,
        response.main.temp_max,
        response.main.feels_like,
        response.main.humidity,
        response.sys.sunrise,
        response.sys.sunset,
        response.coord.lon,
        response.coord.lat
    );
    Ok(())
}
