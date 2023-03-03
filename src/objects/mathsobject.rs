use std::fmt::{Display, Formatter};
use crate::objects::mathsset::MathsSet;
use crate::objects::naturalnumber::NaturalNumber;
use crate::objects::orderedpair::OrderedPair;

#[derive(Clone)]
pub enum MathsObject {
    MathsSet(MathsSet),
    OrderedPair(Box<OrderedPair>),
    NaturalNumber(NaturalNumber),
}

impl ToMathsSet for MathsObject {
    fn to_maths_set(&self) -> MathsSet {
        return match self {
            MathsObject::MathsSet(maths_set) => maths_set.to_maths_set(),
            MathsObject::OrderedPair(ordered_pair) => ordered_pair.to_maths_set(),
            MathsObject::NaturalNumber(natural_number) => natural_number.to_maths_set()
        }
    }
}

impl Display for MathsObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            MathsObject::MathsSet(maths_set) => write!(f, "{}", maths_set),
            MathsObject::OrderedPair(ordered_pair) => write!(f, "{}", ordered_pair),
            MathsObject::NaturalNumber(natural_number) => write!(f, "{}", natural_number)
        };
    }
}

impl PartialEq for MathsObject {
    fn eq(&self, other: &Self) -> bool {
        return self.to_maths_set().eq(&other.to_maths_set());
    }
}

pub trait ToMathsSet {
    fn to_maths_set(&self) -> MathsSet;
}
