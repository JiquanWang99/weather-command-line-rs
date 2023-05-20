mod handler;
use handler::{print_response, Weather};

use exitfailure::ExitFailure;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Input {
    pub city: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // 获取命令行输入的参数，第一个参数即是 city name
    let input = Input::from_args();
    match Weather::get(&input.city).await {
        Ok(r) => print_response(&r),
        Err(e) => println!("请求出错，{:?}", &e),
    };

    Ok(())
}
