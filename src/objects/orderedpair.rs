use std::fmt::{Display, Formatter};
use crate::objects::mathsobject::{MathsObject, ToMathsSet};
use crate::objects::mathsset::MathsSet;

pub struct OrderedPair {
    pub pair: (MathsObject, MathsObject)
}

impl ToMathsSet for OrderedPair {
    fn to_maths_set(&self) -> MathsSet {
        let mut elements = Vec::new();
        let mut left_elements = Vec::new();
        let mut right_elements = Vec::new();
        left_elements.push(self.pair.0.clone());
        right_elements.push(self.pair.0.clone());
        right_elements.push(self.pair.1.clone());
        elements.push( MathsObject::MathsSet(MathsSet { elements: left_elements }));
        elements.push(MathsObject::MathsSet(MathsSet { elements: right_elements }));
        return MathsSet { elements };
    }
}

impl Display for OrderedPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({},{})", self.pair.0, self.pair.1);
    }
}

impl Clone for OrderedPair {
    fn clone(&self) -> Self {
        return OrderedPair { pair: self.pair.clone() };
    }
}
