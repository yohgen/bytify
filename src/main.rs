#![feature(int_log)]

mod cl;

const KIBI: u128 = 1024;
const SIZES: &[&str] = &["Bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];


fn main() {
    let (bytes, precision, huge_indicator) = cl::parse_args();

    let power: u32 = match bytes == 0 {
        true => 0,
        false => bytes.log10() / KIBI.log10(),
    };
    let res: f64 = bytes as f64 / KIBI.pow(power) as f64;
    let size = SIZES[power as usize];

    println!("{}{:.*} {}", huge_indicator, precision, res, size);
}
