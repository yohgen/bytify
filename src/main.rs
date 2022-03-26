#![feature(int_log)]
use std::env;

const KIBI: u128 = 1024;
const SIZES: &[&str] = &["Bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "Yib"];

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let bytes: u64 = match args.get(1) {
        Some(value) => match value.parse() {
            Ok(value) => value,
            Err(err) => panic!("Invalid argument: {}", err),
        },
        None => panic!("Must enter at least one argument"),
    };
    
    let precision: usize = match args.get(2) {
        Some(value) => value.parse::<usize>().unwrap_or(2),
        None => 2,
    };

    let power: u32 = bytes.log10() / KIBI.log10();
    let res: f64 = bytes as f64 / KIBI.pow(power) as f64;
    let size = SIZES[power as usize];

    println!("{:.*} {}", precision, res, size);
}
