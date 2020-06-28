extern crate getopts;
use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Options {
  pub sep: String,
  pub keys: Vec<String>,
  pub has_header: bool,
  pub file: String,
}

fn print_usage(program: &str, opts: getopts::Options) {
  let brief = format!("Usage: {} [options] [FILE]", program);
  print!("{}", opts.usage(&brief));
}

pub fn parse_opts() -> Options {
  let args: Vec<String> = env::args().collect();
  let program = &args[0];
  let mut opts = getopts::Options::new();

  opts.optopt("s", "sep", "line separator. not split if empty", "SEP");
  opts.optopt("k", "keys", "json object keys", "KEYS");
  opts.optflag("", "header", "consider the first line as a header");
  opts.optflag("v", "version", "print version and exit");
  opts.optflag("h", "help", "print usage and exit");

  let matches = opts.parse(&args[1..]).unwrap();

  if matches.opt_present("h") {
    print_usage(&program, opts);
    process::exit(0)
  }

  if matches.opt_present("v") {
    println!("{}", VERSION);
    process::exit(0)
  }

  let sep = matches.opt_get_default("s", ",".to_string()).unwrap();
  let keys_str = matches.opt_get_default("k", "".to_string()).unwrap();
  let has_header = matches.opt_present("header");

  let keys = if keys_str == "" {
    vec![]
  } else {
    keys_str.split(",").map(|s| s.to_string()).collect()
  };

  let file = match matches.free.len() {
    0 => "-".to_string(),
    1 => matches.free[0].to_string(),
    _ => panic!("cannot pass multiple files"),
  };

  Options {
    sep: sep,
    keys: keys,
    has_header: has_header,
    file: file,
  }
}
