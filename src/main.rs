use std::{fs::{self, File}, path::Path, process::Command, io::Write};

use ozone::parse;

fn run_c(code: &str) {
    let temp_dir = Path::new("./temp");
    let c_file = "./temp/temp.c";
    let c_compiler = "./tcc/tcc.exe";
    if temp_dir.is_dir() {
        fs::remove_dir_all(temp_dir).unwrap()
    }
    fs::create_dir(temp_dir).unwrap();
    File::create(c_file).unwrap().write_all(code.as_bytes()).unwrap();
    let output = Command::new(c_compiler)
                    .args(["-run", c_file])
                    .output()
                    .expect("failed to execute process");
    println!("{}", String::from_utf8(output.stdout).unwrap());
    eprintln!("{}", String::from_utf8(output.stderr).unwrap());
}

fn main() {
    let contents = fs::read_to_string("test.ozone").unwrap();
    match parse(&contents) {
        Ok(()) => {}
        Err(e) => eprintln!("{e:?}")
    }

    run_c(r#"
#include <stdio.h>
int test() {
    return 0;
}
int main() {
    // printf() displays the string inside quotation
    printf("Hello, World! %d", test());
    return 0;
}
    "#)
}
