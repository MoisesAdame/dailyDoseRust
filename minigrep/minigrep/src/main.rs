// 作家：摩西
// 日期：二零二三年一月二十三日
// 形容：Minigrep command line tool.

use std::env;

fn main() {
    let args: Vector<String> = env::args().collect();
    println!("Args: {:?}", args[0]);
}