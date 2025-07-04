use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::iter::Peekable;
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
}

impl Pattern {
    pub fn matches(&self, input: &mut Peekable<Chars>, next_pattern: Option<&Pattern>) -> bool {
        match self {
            Pattern::CharLiteral(char) => input.next().is_some_and(|next_char| *char == next_char),

            Pattern::DigitClass => input
                .next()
                .is_some_and(|next_char| is_char_digit(next_char)),

            Pattern::AlphanumericClass => input
                .next()
                .is_some_and(|next_char| is_char_alphanumeric(next_char)),

            Pattern::PositiveCharGroup(char_group) => input
                .next()
                .is_some_and(|next_char| char_group.contains(&next_char)),

            Pattern::NegativeCharGroup(char_group) => input
                .next()
                .is_some_and(|next_char| !char_group.contains(&next_char)),

            Pattern::StartOfString(start_pattern) => start_pattern.matches(input, None),

            Pattern::EndOfString(end_pattern) => {
                end_pattern.matches(input, None) && input.next().is_none()
            }

            Pattern::OneOrMoreQuantifier(current_pattern) => {
                if !current_pattern.matches(input, None) {
                    return false;
                }

                match next_pattern {
                    // Advance input iterator
                    None => while current_pattern.matches(input, None) {},

                    Some(nxt_pattern) => {
                        let input_clone = &mut input.clone();
                        let mut to_advance = 0;

                        if **current_pattern == *nxt_pattern {
                            while input_clone.peek().is_some()
                                && current_pattern.matches(input_clone, None)
                            {
                                to_advance += 1;
                            }
                            to_advance -= 1;
                        } else {
                            while input_clone.peek().is_some()
                                && !nxt_pattern.matches(input_clone, None)
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
                            if char_literal == *next_char.unwrap() {
                                input.next();
                            }
                        }
                        _ => {
                            optional_pattern.matches(input, None);
                        }
                    }
                }

                true
            }

            Pattern::Wildcard => input.next().is_some(),

            Pattern::Group(group) => group.iter().all(|pattern| pattern.matches(input, None)),

            Pattern::Alternation(alternation) => {
                for variant in alternation {
                    let input_clone = &mut input.clone();
                    let variant_length = variant.len();

                    if variant
                        .iter()
                        .all(|pattern| pattern.matches(input_clone, None))
                    {
                        for _ in 0..variant_length {
                            input.next();
                        }

                        return true;
                    }
                }

                false
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
