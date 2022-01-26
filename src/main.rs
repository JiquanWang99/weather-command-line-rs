mod module;
use module::core::{print_response, Input, Weather};

use colored::*;
use exitfailure::ExitFailure;
use structopt::StructOpt;

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
