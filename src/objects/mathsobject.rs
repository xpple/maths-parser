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
        };
    }
}

impl Display for MathsObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            MathsObject::MathsSet(maths_set) => maths_set.fmt(f),
            MathsObject::OrderedPair(ordered_pair) => ordered_pair.fmt(f),
            MathsObject::NaturalNumber(natural_number) => natural_number.fmt(f)
        };
    }
}

impl PartialEq for MathsObject {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (MathsObject::MathsSet(self_maths_set), MathsObject::MathsSet(other_maths_set)) => {
                return self_maths_set.eq(&other_maths_set);
            },
            (MathsObject::OrderedPair(self_ordered_pair), MathsObject::OrderedPair(other_ordered_pair)) => {
                return self_ordered_pair.pair.eq(&other_ordered_pair.pair);
            },
            (MathsObject::NaturalNumber(self_natural_number), MathsObject::NaturalNumber(other_natural_number)) => {
                return self_natural_number.natural_number.eq(&other_natural_number.natural_number);
            },
            (MathsObject::OrderedPair(_), MathsObject::NaturalNumber(_)) => false,
            (MathsObject::NaturalNumber(_), MathsObject::OrderedPair(_)) => false,
            (_, _) => self.to_maths_set().eq(&other.to_maths_set())
        };
    }
}

pub trait ToMathsSet {
    fn to_maths_set(&self) -> MathsSet;
}
