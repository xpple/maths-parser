use crate::objects::mathsobject::ToMathsSet;
use crate::parser::mathsparser::MathsParser;

mod parser;
mod objects;

fn main() {
    println!("Enter your mathematical object below.");
    let mut maths_string = String::new();
    std::io::stdin().read_line(&mut maths_string).unwrap();
    let maths_object = MathsParser::from_string(&maths_string).unwrap();
    println!("{}", maths_object);
}
