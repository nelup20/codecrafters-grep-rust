use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;
const ASCII_UPPERCASE_A: u8 = 65;
const ASCII_UPPERCASE_Z: u8 = 90;
const ASCII_LOWERCASE_A: u8 = 97;
const ASCII_LOWERCASE_Z: u8 = 122;

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    CharLiteral(char),
    DigitClass,
    AlphanumericClass,
    PositiveCharGroup(Vec<char>),
    NegativeCharGroup(Vec<char>),
    StartOfString(Box<Pattern>),
    EndOfString(Box<Pattern>),
    OneOrMoreQuantifier(Box<Pattern>),
    OptionalQuantifier(Box<Pattern>),
    Wildcard,
    Group(VecDeque<Pattern>),
    Alternation(Vec<Vec<Pattern>>),
    Backreference(usize),
}

impl Pattern {
    pub fn matches(
        &self,
        input: &mut Peekable<Enumerate<Chars>>,
        next_pattern: Option<&Pattern>,
        backreference_values: &mut Vec<String>,
    ) -> bool {
        match self {
            Pattern::CharLiteral(char) => input
                .next()
                .is_some_and(|(_, next_char)| *char == next_char),

            Pattern::DigitClass => input
                .next()
                .is_some_and(|(_, next_char)| is_char_digit(next_char)),

            Pattern::AlphanumericClass => input
                .next()
                .is_some_and(|(_, next_char)| is_char_alphanumeric(next_char)),

            Pattern::PositiveCharGroup(char_group) => input
                .next()
                .is_some_and(|(_, next_char)| char_group.contains(&next_char)),

            Pattern::NegativeCharGroup(char_group) => input
                .next()
                .is_some_and(|(_, next_char)| !char_group.contains(&next_char)),

            Pattern::StartOfString(start_pattern) => {
                start_pattern.matches(input, next_pattern, backreference_values)
            }

            Pattern::EndOfString(end_pattern) => {
                end_pattern.matches(input, next_pattern, backreference_values)
                    && input.next().is_none()
            }

            Pattern::OneOrMoreQuantifier(current_pattern) => {
                if !current_pattern.matches(input, next_pattern, backreference_values) {
                    return false;
                }

                match next_pattern {
                    // Advance input iterator
                    None => while current_pattern.matches(input, None, backreference_values) {},

                    Some(nxt_pattern) => {
                        let input_clone = &mut input.clone();
                        let mut to_advance = 0;

                        if **current_pattern == *nxt_pattern {
                            while input_clone.peek().is_some()
                                && current_pattern.matches(input_clone, None, backreference_values)
                            {
                                to_advance += 1;
                            }
                            to_advance -= 1;
                        } else {
                            while input_clone.peek().is_some()
                                && !nxt_pattern.matches(input_clone, None, backreference_values)
                            {
                                to_advance += 1;
                            }
                        }

                        for _ in 0..to_advance {
                            input.next();
                        }
                    }
                }

                true
            }

            Pattern::OptionalQuantifier(optional_pattern) => {
                let next_char = input.peek();
                if next_char.is_some() {
                    match **optional_pattern {
                        Pattern::CharLiteral(char_literal) => {
                            if char_literal == next_char.unwrap().1 {
                                input.next();
                            }
                        }
                        _ => {
                            optional_pattern.matches(input, None, backreference_values);
                        }
                    }
                }

                true
            }

            Pattern::Wildcard => input.next().is_some(),

            Pattern::Group(group) => {
                // For backreferences:
                // save input, index and length before advancing the group and add new backref value
                let mut input_clone = input.clone();
                let (input_index_before, _) = input_clone.peek().unwrap_or_else(|| &(0usize, '\0'));
                let backreferences_length = backreference_values.len();
                backreference_values.push(String::new());

                let mut group_patterns = group.iter().enumerate().peekable();
                let mut next_group_pattern: Option<&Pattern> = None;

                while let Some((i, group_pattern)) = group_patterns.next() {
                    // No more patterns left, so we need the pattern outside the group
                    if i == group.len() - 1 {
                        next_group_pattern = next_pattern;
                    } else {
                        next_group_pattern = Some(group_patterns.peek().unwrap().1);
                    }

                    if !group_pattern.matches(input, next_group_pattern, backreference_values) {
                        return false;
                    }
                }

                // Capture the group's matched value
                if let Some((input_index_after, _)) = input.peek() {
                    let mut group_backreference = String::new();

                    for _ in 0..(*input_index_after - *input_index_before) {
                        group_backreference.push(input_clone.next().unwrap().1);
                    }

                    backreference_values.remove(backreferences_length);
                    backreference_values.insert(backreferences_length, group_backreference);
                }

                true
            }

            Pattern::Alternation(alternation) => {
                for variant in alternation {
                    let input_clone = &mut input.clone();
                    let variant_length = variant.len();

                    if variant
                        .iter()
                        .all(|pattern| pattern.matches(input_clone, None, backreference_values))
                    {
                        for _ in 0..variant_length {
                            input.next();
                        }

                        return true;
                    }
                }

                false
            }

            Pattern::Backreference(nth) => {
                let backreference = backreference_values.get(nth - 1).unwrap();

                for backreference_char in backreference.chars() {
                    if input
                        .next()
                        .is_none_or(|(_, input_char)| input_char != backreference_char)
                    {
                        return false;
                    }
                }

                true
            }
        }
    }
}

#[inline(always)]
fn is_char_alphanumeric(char: char) -> bool {
    is_char_digit(char) || is_char_letter(char)
}

#[inline(always)]
fn is_char_letter(char: char) -> bool {
    is_char_uppercase_letter(char) || is_char_lowercase_letter(char)
}

#[inline(always)]
fn is_char_uppercase_letter(char: char) -> bool {
    char as u8 >= ASCII_UPPERCASE_A && char as u8 <= ASCII_UPPERCASE_Z
}

#[inline(always)]
fn is_char_lowercase_letter(char: char) -> bool {
    char as u8 >= ASCII_LOWERCASE_A && char as u8 <= ASCII_LOWERCASE_Z
}

#[inline(always)]
fn is_char_digit(char: char) -> bool {
    char as u8 >= ASCII_0 && char as u8 <= ASCII_9
}
