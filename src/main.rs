mod cli;
mod xjr;

use cli::parse_opts;
use xjr::each_json_line;

fn main() {
    let opts = parse_opts();

    each_json_line(&opts.file, &opts.sep, &opts.keys, opts.has_header, |line| {
        println!("{}", line)
    })
    .unwrap();
}
