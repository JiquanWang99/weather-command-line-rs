use chrono::prelude::*;
use colored::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, UNIX_EPOCH};
use structopt::StructOpt;

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

fn print_response(resp: &Weather) {
    println!(
        "  当前温度：{}℃ \n  今日最低温：{}℃ \n  今日最高温：{}℃ \n  体感温度：{}℃ \n  湿度：{}% \n  日出时间：{} \n  日落时间：{} \n  当前经度：{} \n  当前纬度：{}",
        resp.main.temp.to_string().bright_red(),
        resp.main.temp_min,
        resp.main.temp_max,
        resp.main.feels_like,
        resp.main.humidity,
        formate_timestamp(resp.sys.sunrise),
        formate_timestamp(resp.sys.sunset),
        resp.coord.lon,
        resp.coord.lat
    );
}

fn formate_timestamp(timestamp: i32) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let time = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Local>::from(time);
    datetime.format("%H:%M:%S").to_string()
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    println!("{}", "-----正在获取天气信息, 请稍后...-----".bright_green());
    match Weather::get(&input.city).await {
        Ok(r) => print_response(&r),
        Err(e) => println!("请求出错，{:?}", &e),
    };

    Ok(())
}
