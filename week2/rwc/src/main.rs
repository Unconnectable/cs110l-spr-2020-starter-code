use std::env;
use std::process;
use std::fs::File;
use std::io::{ self, BufRead };
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    let mut lines_count = 0;
    let mut words_count = 0;
    let mut chars_count = 0;

    let file = match File::open(filename) {
        Ok(file_) => file_,
        Err(_e) => {
            //编译器提示修改变量名称
            println!("File has Error");
            process::exit(1);
        }
    };
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        // 直接line? 会得到类型错误
        let line_str = line.expect("line error");
        lines_count += 1;
        chars_count += line_str.len();
        words_count += line_str.split_whitespace().count();
    }
    println!("lines: {}", lines_count);
    println!("words: {}", words_count);
    println!("chars: {}", chars_count);
    // Your code here :)
}