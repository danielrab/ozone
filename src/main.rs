use std::fs;

use ozone::parse;

fn main() {
    let contents = fs::read_to_string("test.ozone").unwrap();
    let tokens = parse(&contents).unwrap();
    println!("{tokens:#?}")
}
