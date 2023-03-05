use std::collections::HashMap;
use crate::objects::mathsobject::MathsObject;
use crate::objects::mathsset::MathsSet;
use crate::objects::naturalnumber::NaturalNumber;
use crate::objects::orderedpair::OrderedPair;

pub struct MathsParser {
}

impl MathsParser {
    pub fn from_string(maths_string: &str) -> Option<MathsObject> {
        let chars = maths_string.trim().chars().collect::<Vec<_>>();
        return MathsParser::parse_maths_object(&chars);
    }

    fn parse_maths_object(chars: &[char]) -> Option<MathsObject> {
        if chars.is_empty() {
            return None;
        }
        let char = chars[0];
        if char == '{' {
            let j = MathsParser::find_closing(chars, '{');
            return Some(MathsObject::MathsSet(MathsParser::parse_maths_set(&chars[1..j])));
        }
        if char == '(' {
            let j = MathsParser::find_closing(chars, '(');
            return Some(MathsObject::OrderedPair(Box::new(MathsParser::parse_ordered_pair(&chars[1..j]))));
        }
        return Some(MathsObject::NaturalNumber(NaturalNumber { natural_number: chars.iter().collect::<String>().parse::<i32>().unwrap() }));
    }

    fn parse_maths_set(chars: &[char]) -> MathsSet {
        let mut elements = Vec::new();
        let mut start = 0;
        while start < chars.len() {
            let end = MathsParser::find_closing(&chars[start..], ',') + start;
            if let Some(element) = MathsParser::parse_maths_object(&chars[start..end]) {
                if !elements.contains(&element) {
                    elements.push(element);
                }
            }
            start = end + 1;
        }
        return MathsSet { elements };
    }

    fn parse_ordered_pair(chars: &[char]) -> OrderedPair {
        let j = MathsParser::find_closing(chars, ',');
        let left = MathsParser::parse_maths_object(&chars[..j]).unwrap();
        let right = MathsParser::parse_maths_object(&chars[j + 1..]).unwrap();
        return OrderedPair { pair: (left, right) };
    }

    pub fn replace_natural_numbers(maths_object: MathsObject) -> String {
        let mut set_string = maths_object.to_string();
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
