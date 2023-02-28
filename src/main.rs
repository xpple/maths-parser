use crate::mathsparser::MathsObject;

mod mathsparser;

fn main() {
    println!("Enter your mathematical object below.");
    let mut maths_string: String = String::new();
    std::io::stdin().read_line(&mut maths_string).unwrap();
    let maths_set = MathsObject::from_string(&maths_string).unwrap();
    println!("{}", maths_set);
    println!("{}", maths_set.replace_natural_numbers());
}
