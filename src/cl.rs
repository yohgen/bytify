use std::env;

const DEF_PR: usize = 2;
const MAX_PR: usize = 8;
const MAX_BYTES: u64 = 18446744073709499999;
const ARG_B: &str = "--bytes=";
const ARG_P: &str = "--precision=";
const ARG_B_SH: &str = "-b";
const ARG_P_SH: &str = "-p";

pub fn parse_args() -> (u64, usize) {
  let args: Vec<String> = env::args().collect();
  let mut bytes: u64 = 0;
  let mut precision: usize = 2;

  for idx in 0..args.len() {
    let arg = &args[idx];

    if arg == ARG_B_SH || arg.starts_with(ARG_B) {
      let intermediate = match arg == ARG_B_SH {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(ARG_B).unwrap().to_string(),
      };

      bytes = intermediate
        .parse::<u64>()
        .expect("Invalid argument")
        .clamp(0, MAX_BYTES);

    } else if arg == ARG_P_SH || arg.starts_with(ARG_P) {
      let intermediate = match arg == ARG_P_SH {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(ARG_P).unwrap().to_string(),
      };

      precision = intermediate
        .parse::<usize>()
        .unwrap_or(DEF_PR)
        .clamp(0, MAX_PR);

    }
  }

  (bytes, precision)
}