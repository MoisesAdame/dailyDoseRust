// 作家：摩西
// 日期：二零二三年一月二十三日
// 形容：Minigrep command line tool.

extern crate minigrep;
use minigrep::Config;

use std::env;
use std::process;



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Check if the return type is an Error.
    if let Err(e) = minigrep::run(config){
        println!("Application error: {:?}", e);
        process::exit(1);
    }
}