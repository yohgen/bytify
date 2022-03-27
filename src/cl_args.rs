use std::env;

const DEF_PR: usize = 2;
const MIN_MAX_PR: (usize, usize) = (0, 8);
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
        false => arg.strip_prefix(ARG_P).unwrap().to_string(),
      };

      bytes = intermediate.parse().expect("Invalid argument");

    } else if arg == ARG_P_SH || arg.starts_with(ARG_P) {
      let intermediate = match arg == ARG_P_SH {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(ARG_P).unwrap().to_string(),
      };

      precision = intermediate
        .parse()
        .unwrap_or(DEF_PR)
        .clamp(MIN_MAX_PR.0, MIN_MAX_PR.1);

    }
  }

  (bytes, precision)
}