extern crate dhdl_parser;

use std::env;
use dhdl_parser::parse_file;

fn main()
{
    let args = env::args();
    let fname = args.skip(1).next().unwrap();

    parse_file(&fname);
}
