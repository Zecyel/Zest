// #[path ="./util/stream.rs"]
// pub mod stream;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[path = "./core/parser.rs"]
pub mod parser;

fn main() {
    // println!("Hello, world!");
    let a = "(hello (plus) (name 1 (1 2 3)))";
    println!("{:#?}", parser::parse_zest_file(a).expect("Error!"));
}
