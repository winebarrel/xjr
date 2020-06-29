mod cli;
mod xjr;

use cli::parse_opts;
use std::fs;
use std::io;
use xjr::each_json_line;

fn main() {
    let opts = parse_opts();
    let cb = |line| println!("{}", line);

    if opts.file == "-" {
        let mut reader = io::BufReader::new(io::stdin());
        each_json_line(&mut reader, &opts.sep, &opts.keys, opts.has_header, cb).unwrap();
    } else {
        let f = fs::File::open(opts.file).unwrap();
        let mut reader = io::BufReader::new(f);
        each_json_line(&mut reader, &opts.sep, &opts.keys, opts.has_header, cb).unwrap();
    }
}
