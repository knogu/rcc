use std::env;
mod parse;
use crate::parse::parse;
mod generate;


fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = parse(&args[1]).unwrap();
    generate::generate(parsed);
}
