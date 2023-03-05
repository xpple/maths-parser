use std::fmt::{Display, Formatter};
use crate::objects::mathsobject::{MathsObject, ToMathsSet};
use crate::objects::mathsset::MathsSet;

#[derive(Clone)]
pub struct NaturalNumber {
    pub natural_number: u32
}

impl ToMathsSet for NaturalNumber {
    fn to_maths_set(&self) -> MathsSet {
        if self.natural_number == 0 {
            return MathsSet { elements: Vec::new() }
        }
        let mut predecessor = NaturalNumber { natural_number: self.natural_number - 1 }.to_maths_set();
        predecessor.elements.push(MathsObject::MathsSet(predecessor.clone()));
        return predecessor;
    }
}

impl Display for NaturalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return self.natural_number.fmt(f);
    }
}
