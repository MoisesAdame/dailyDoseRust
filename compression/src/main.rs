// 作家：摩西
// 日期：二零二三年一月二十五日
// Description：CLI main doc for the compression algorithms.

use std::env;
use std::process;

extern crate compression;
use compression::*;

fn main(){
    let args: Vec<String> = env::args().collect();

    let query = Query::new(&args);

    let query = Query::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });



    run(&query);
}