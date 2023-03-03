#![allow(clippy::needless_return)]

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
pub enum MathsObject {
    MathsSet(MathsSet),
    OrderedPair(Box<OrderedPair>),
    Number(i32),
}

pub trait AsMathsSet {
    fn as_maths_set(&self) -> MathsSet;
}

impl Display for MathsObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            MathsObject::MathsSet(maths_set) => write!(f, "{}", maths_set),
            MathsObject::OrderedPair(ordered_pair) => write!(f, "{}", ordered_pair),
            MathsObject::Number(number) => write!(f, "{}", number)
        };
    }
}

impl AsMathsSet for MathsObject {
    fn as_maths_set(&self) -> MathsSet {
        return match self {
            MathsObject::MathsSet(maths_set) => maths_set.as_maths_set(),
            MathsObject::OrderedPair(ordered_pair) => ordered_pair.as_maths_set(),
            MathsObject::Number(number) => number.as_maths_set(),
        }
    }
}

pub struct MathsSet {
    elements: Vec<MathsObject>
}

impl MathsSet {
    pub fn as_ordered_pair(&self) -> Option<OrderedPair> {
        if self.elements.len() != 2 {
            return None;
        }

        let left = &self.elements[0];
        let left_set = match left {
            MathsObject::MathsSet(left_set) => left_set,
            _ => return None
        };
        let right = &self.elements[1];
        let right_set = match right {
            MathsObject::MathsSet(right_set) => right_set,
            _ => return None
        };

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
        let elements = &self.elements;
        return MathsSet { elements: elements.clone() };
    }
}

impl AsMathsSet for MathsSet {
    fn as_maths_set(&self) -> Self {
        return self.clone();
    }
}

pub struct OrderedPair {
    pair: (MathsObject, MathsObject)
}

impl Display for OrderedPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({},{})", self.pair.0, self.pair.1);
    }
}

impl PartialEq for OrderedPair {
    fn eq(&self, other: &Self) -> bool {
        return self.pair == other.pair;
    }
}

impl Clone for OrderedPair {
    fn clone(&self) -> Self {
        let pair = &self.pair;

        return OrderedPair { pair: pair.clone() };
    }
}

impl AsMathsSet for OrderedPair {
    fn as_maths_set(&self) -> MathsSet {
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

impl AsMathsSet for i32 {
    fn as_maths_set(&self) -> MathsSet {
        if *self == 0 {
            return MathsSet { elements: Vec::new() }
        }
        let mut predecessor = (self - 1).as_maths_set();
        predecessor.elements.push(MathsObject::MathsSet(predecessor.clone()));
        return predecessor;
    }
}

impl MathsObject {
    pub fn from_string(maths_string: &str) -> Option<MathsObject> {
        let chars = maths_string.trim().chars().collect::<Vec<_>>();
        return MathsObject::parse_maths_object(&chars);
    }

    fn parse_maths_object(chars: &[char]) -> Option<MathsObject> {
        if chars.is_empty() {
            return None;
        }
        let char = chars[0];
        if char == '{' {
            let j = MathsObject::find_closing(&chars, '{');
            let maths_set = MathsObject::parse_maths_set(&chars[1..j]);
            return match maths_set.as_ordered_pair() {
                Some(ordered_pair) => Some(MathsObject::OrderedPair(Box::new(ordered_pair))),
                //Some(_) => Some(MathsObject::MathsSet(maths_set)),
                None => Some(MathsObject::MathsSet(maths_set))
            };
        }
        if char == '(' {
            let j = MathsObject::find_closing(&chars, '(');
            return Some(MathsObject::OrderedPair(Box::new(MathsObject::parse_ordered_pair(&chars[1..j]))));
        }
        return Some(MathsObject::Number(chars.iter().collect::<String>().parse::<i32>().unwrap()));
    }

    fn parse_maths_set(chars: &[char]) -> MathsSet {
        let mut elements: Vec<MathsObject> = Vec::new();
        let mut start = 0;
        while start < chars.len() {
            let end = MathsObject::find_closing(&chars[start..], ',') + start;
            if let Some(element) = MathsObject::parse_maths_object(&chars[start..end]) {
                if !elements.contains(&element) {
                    elements.push(element);
                }
            }
            start = end + 1;
        }
        return MathsSet { elements };
    }

    fn parse_ordered_pair(chars: &[char]) -> OrderedPair {
        let j = MathsObject::find_closing(&chars, ',');
        let a = MathsObject::parse_maths_object(&chars[..j]).unwrap();
        let b = MathsObject::parse_maths_object(&chars[j + 1..]).unwrap();
        return OrderedPair { pair: (a, b) };
    }

    pub fn replace_natural_numbers(&self) -> String {
        let mut set_string = self.to_string();
        let mut replacement = String::new();
        let mut number = 0;
        loop {
            let number_string = set_string.replace(&format!("{{{}}}", &replacement), &number.to_string());
            if number_string == set_string {
                return number_string;
            }
            number += 1;
            replacement = (0..number).into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
            set_string = number_string;
        }
    }

    fn find_closing(chars: &[char], next: char) -> usize {
        let close_brackets: HashMap<char, char> = HashMap::from([('{', '}'), ('(', ')')]);
        let open_brackets: HashMap<char, char> = close_brackets.iter().map(|(&k, &v)| (v, k)).collect();
        let closing = match close_brackets.get(&next) {
            Some(closing) => closing,
            None => &next
        };
        let mut stack = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            let char = chars[i];
            if close_brackets.get(&char).is_some() {
                stack.push(char);
            }
            if open_brackets.get(&char).is_some() {
                stack.pop();
            }
            if char == *closing && stack.is_empty() {
                return i;
            }
            i += 1;
        }
        return i;
    }
}
