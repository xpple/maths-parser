use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum MathsObject {
    MathsSet(MathsSet),
    OrderedPair(Box<OrderedPair>),
    Number(i32),
}

impl Display for MathsObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MathsObject::MathsSet(maths_set) => {write!(f, "{}", maths_set)}
            MathsObject::OrderedPair(ordered_pair) => {write!(f, "{}", ordered_pair)}
            MathsObject::Number(number) => {write!(f, "{}", number)}
        }
    }
}

impl Clone for MathsObject {
    fn clone(&self) -> Self {
        match self {
            MathsObject::MathsSet(maths_set) => {
                let elements = &maths_set.elements;

                return MathsObject::MathsSet(MathsSet { elements: elements.clone() });
            },
            MathsObject::OrderedPair(ordered_pair) => {
                let a = &ordered_pair.a;
                let b = &ordered_pair.b;

                return MathsObject::OrderedPair(Box::new(OrderedPair { a: a.clone(), b: b.clone() }))
            },
            MathsObject::Number(number) => MathsObject::Number(*number)
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
            return Some(OrderedPair { a: left_element.clone(), b: right_element_two.clone() });
        }
        if left_element == right_element_two {
            return Some(OrderedPair { a: left_element.clone(), b: right_element_one.clone() });
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
                string.push_str(",");
            }
        }
        write!(f, "{{{}}}", string)
    }
}

impl PartialEq<Self> for MathsSet {
    fn eq(&self, other: &Self) -> bool {
        if !self.elements.iter().all(|element| other.elements.contains(element)) {
            return false
        }
        if !other.elements.iter().all(|element| self.elements.contains(element)) {
            return false
        }
        return true
    }
}

pub struct OrderedPair {
    a: MathsObject,
    b: MathsObject
}

impl OrderedPair {
    pub fn as_maths_set(&self) -> MathsSet {
        let mut elements = Vec::new();
        let mut left_elements = Vec::new();
        let mut right_elements = Vec::new();
        left_elements.push(self.a.clone());
        right_elements.push(self.a.clone());
        right_elements.push(self.b.clone());
        elements.push( MathsObject::MathsSet(MathsSet { elements: left_elements }));
        elements.push(MathsObject::MathsSet(MathsSet { elements: right_elements }));
        return MathsSet { elements }
    }
}

impl Display for OrderedPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.a, self.b)
    }
}

impl PartialEq for OrderedPair {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl MathsObject {
    pub fn from_string(maths_string: &str) -> Option<MathsObject> {
        let chars = maths_string.trim().chars().collect::<Vec<_>>();
        MathsObject::parse_maths_object(&chars)
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
                Some(ordered_pair) => Some(MathsObject::OrderedPair(Box::from(ordered_pair))),
                //Some(_) => Some(MathsObject::MathsSet(maths_set)),
                None => Some(MathsObject::MathsSet(maths_set))
            };
        }
        if char == '(' {
            let j = MathsObject::find_closing(&chars, '(');
            return Some(MathsObject::OrderedPair(Box::new(MathsObject::parse_ordered_pair(&chars[1..j]))));
        }
        return Some(MathsObject::Number(chars.iter().collect::<String>().parse::<i32>().unwrap()))
    }

    fn parse_maths_set(chars: &[char]) -> MathsSet {
        let mut elements: Vec<MathsObject> = Vec::new();
        let mut start = 0;
        while start < chars.len() {
            let end = MathsObject::find_closing(&chars[start..], ',') + start;
            match MathsObject::parse_maths_object(&chars[start..end]) {
                Some(element) => {
                    if !elements.contains(&element) {
                        elements.push(element)
                    }
                },
                None => {}
            }
            start = end + 1
        }
        return MathsSet { elements };
    }

    fn parse_ordered_pair(chars: &[char]) -> OrderedPair {
        let j = MathsObject::find_closing(&chars, ',');
        let a = MathsObject::parse_maths_object(&chars[..j]).unwrap();
        let b = MathsObject::parse_maths_object(&chars[j + 1..]).unwrap();
        return OrderedPair { a, b };
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
            match close_brackets.get(&char) {
                Some(_) => stack.push(char),
                None => {}
            };
            match open_brackets.get(&char) {
                Some(_) => stack.pop(),
                None => None
            };
            if char == *closing && stack.is_empty() {
                return i;
            }
            i += 1
        }
        return i;
    }
}