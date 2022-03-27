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

    if arg == BYTES_ARG_SHORT || arg.starts_with(BYTES_ARG) {
      let intermediate = match arg == BYTES_ARG_SHORT {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(PRECISION_ARG).unwrap().to_string(),
      };

      bytes = intermediate.parse().expect("Invalid argument");

    } else if arg == PRECISION_ARG_SHORT || arg.starts_with(PRECISION_ARG) {
      let intermediate = match arg == PRECISION_ARG_SHORT {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(PRECISION_ARG).unwrap().to_string(),
      };

      precision = intermediate
        .parse()
        .unwrap_or(DEFAULT_PR)
        .clamp(MIN_MAX_PR.0, MIN_MAX_PR.1);

    }
  }

  (bytes, precision)
}