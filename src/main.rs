#[macro_use]
extern crate duct;

mod interpreter;
use interpreter::LSInterpreter;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ls_file = File::open(&args[1]).expect("File not found");
    let mut interpreter = LSInterpreter::new(ls_file);
    interpreter.run();
}