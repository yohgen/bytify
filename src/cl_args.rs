use std::env;

const DEFAULT_PR: usize = 2;
const MIN_MAX_PR: (usize, usize) = (0, 8);
const BYTES_ARG: &str = "--bytes=";
const PRECISION_ARG: &str = "--precision=";
const BYTES_ARG_SHORT: &str = "-b";
const PRECISION_ARG_SHORT: &str = "-p";

pub fn parse_args() -> (u64, usize) {
  let args: Vec<String> = env::args().collect();
  let mut bytes: u64 = 0;
  let mut precision: usize = 2;

  for idx in 0..args.len() {
    let arg = &args[idx];

    if arg == BYTES_ARG_SHORT {
      bytes = args[idx + 1].parse().expect("Invalid argument");

    } else if arg.starts_with(BYTES_ARG) {
      bytes = arg
        .strip_prefix(BYTES_ARG)
        .unwrap()
        .parse()
        .expect("Invalid argument");

    } else if arg == PRECISION_ARG_SHORT {
      precision = args[idx + 1].parse().unwrap_or(DEFAULT_PR);

    } else if arg.starts_with(PRECISION_ARG) {
      precision = arg
        .strip_prefix(PRECISION_ARG)
        .unwrap()
        .parse()
        .unwrap_or(DEFAULT_PR)
        .clamp(MIN_MAX_PR.0, MIN_MAX_PR.1);
    }
  }

  (bytes, precision)
}