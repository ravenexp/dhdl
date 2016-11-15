extern crate dhdl_parser;

const TESTFILE: &'static str = "tests/module-0.dhdl";

#[test]
fn test_parse_file()
{
        dhdl_parser::parse_file(TESTFILE);
}
