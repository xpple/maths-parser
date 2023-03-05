use std::fmt::{Display, Formatter};
use crate::objects::mathsobject::{MathsObject, ToMathsSet};
use crate::objects::orderedpair::OrderedPair;

pub struct MathsSet {
    pub elements: Vec<MathsObject>
}

impl MathsSet {
    pub fn to_ordered_pair(&self) -> Option<OrderedPair> {
        if self.elements.len() != 2 {
            return None;
        }

        let left_set = &self.elements[0].to_maths_set();
        let right_set = &self.elements[1].to_maths_set();

        let left_element;
        let right_element_one;
        let right_element_two;

        if left_set.elements.len() == 1 && right_set.elements.len() == 2 {
            left_element = &left_set.elements[0];
            right_element_one = &right_set.elements[0];
            right_element_two = &right_set.elements[1];
        } else if left_set.elements.len() == 2 && right_set.elements.len() == 1 {
            left_element = &right_set.elements[0];
            right_element_one = &left_set.elements[0];
            right_element_two = &left_set.elements[1];
        } else {
            return None;
        }

        if left_element == right_element_one {
            return Some(OrderedPair { pair: (left_element.clone(), right_element_two.clone()) });
        }
        if left_element == right_element_two {
            return Some(OrderedPair { pair: (left_element.clone(), right_element_one.clone()) });
        }
        return None;
    }
}

impl ToMathsSet for MathsSet {
    fn to_maths_set(&self) -> MathsSet {
        return self.clone();
    }
}

impl Display for MathsSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for (index, element) in self.elements.iter().enumerate() {
            string.push_str(&element.to_string());
            if index < self.elements.len() - 1 {
                string.push(',');
            }
        }
        return write!(f, "{{{}}}", string);
    }
}

impl PartialEq<Self> for MathsSet {
    fn eq(&self, other: &Self) -> bool {
        if !self.elements.iter().all(|element| other.elements.contains(element)) {
            return false;
        }
        if !other.elements.iter().all(|element| self.elements.contains(element)) {
            return false;
        }
        return true;
    }
}

impl Clone for MathsSet {
    fn clone(&self) -> Self {
        return MathsSet { elements: self.elements.clone() };
    }
}
