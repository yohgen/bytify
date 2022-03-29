use std::env;
use std::num::IntErrorKind;

const DEF_P: usize = 2;
const MAX_P: usize = 8;
const MAX_BYTES: u128 = 1024u128.pow(9) - 1;
const ARG_B: &str = "--bytes=";
const ARG_P: &str = "--precision=";
const ARG_B_SH: &str = "-b";
const ARG_P_SH: &str = "-p";

pub fn parse_args() -> (u128, usize, String) {
  let args: Vec<String> = env::args().collect();
  let mut bytes: u128 = 0;
  let mut precision: usize = 2;
  let mut huge_indicator: String = "".to_owned();

  for idx in 0..args.len() {
    let arg = &args[idx];

    if arg == ARG_B_SH || arg.starts_with(ARG_B) {
      let intermediate = match arg == ARG_B_SH {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(ARG_B).unwrap().to_owned(),
      };

      bytes = match intermediate.parse::<u128>() {
        Ok(value) => if value > MAX_BYTES {
          huge_indicator = "+".to_owned();

          MAX_BYTES
        } else {
          value
        },
        Err(err) => match err.kind() {
          IntErrorKind::InvalidDigit => panic!("Bytes must be a number"),
          IntErrorKind::PosOverflow => {
            huge_indicator = "+".to_owned();

            MAX_BYTES
          }
          _ => panic!("Invalid bytes"),
        }
      }
    } else if arg == ARG_P_SH || arg.starts_with(ARG_P) {
      let intermediate = match arg == ARG_P_SH {
        true => args[idx + 1].to_owned(),
        false => arg.strip_prefix(ARG_P).unwrap().to_owned(),
      };

      precision = intermediate
        .parse::<usize>()
        .unwrap_or(DEF_P)
        .clamp(0, MAX_P);

    }
  }

  (bytes, precision, huge_indicator)
}