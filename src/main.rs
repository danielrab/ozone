use std::fs;

use ozone::parse;

fn main() {
    let contents = fs::read_to_string("test.ozone").unwrap();
    parse(contents).unwrap();
    //println!("{tokens:#?}")
}
