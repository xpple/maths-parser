use crate::mathsparser::{AsMathsSet, MathsObject};

mod mathsparser;

fn main() {
    println!("Enter your mathematical object below.");
    let mut maths_string: String = String::new();
    std::io::stdin().read_line(&mut maths_string).unwrap();
    let maths_object = MathsObject::from_string(&maths_string).unwrap();
    println!("{}", maths_object);
    println!("{}", maths_object.replace_natural_numbers());
    println!("{}", maths_object.as_maths_set())
}
