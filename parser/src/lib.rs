extern crate lalrpop_util;

pub mod ast;
mod parser;

use std::fs::File;
use std::io::Read;

pub fn parse_file(name: &str) -> ast::ModuleDecl
{
    let mut file = File::open(name).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    parser::parse_ModuleDecl(&code).unwrap()
}
