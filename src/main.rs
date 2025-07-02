use std::env;
use std::io;
use std::process;

const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;
const ASCII_UPPERCASE_A: u8 = 65;
const ASCII_UPPERCASE_Z: u8 = 90;
const ASCII_LOWERCASE_A: u8 = 97;
const ASCII_LOWERCASE_Z: u8 = 122;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        "\\d" => {
            input_line.chars()
                .find(|&char| char_is_digit(char))
                .is_some()
        },
        
        "\\w" => {
            input_line.chars()
                .find(|&char| char_is_digit(char) || char_is_letter(char))
                .is_some()
        }
        char => input_line.contains(char),
    }
}

#[inline(always)]
fn char_is_letter(char: char) -> bool {
    char_is_uppercase_letter(char) || char_is_lowercase_letter(char)
}

#[inline(always)]
fn char_is_uppercase_letter(char: char) -> bool {
    char as u8 >= ASCII_UPPERCASE_A && char as u8 <= ASCII_UPPERCASE_Z
}

#[inline(always)]
fn char_is_lowercase_letter(char: char) -> bool {
    char as u8 >= ASCII_LOWERCASE_A && char as u8 <= ASCII_LOWERCASE_Z
}

#[inline(always)]
fn char_is_digit(char: char) -> bool {
    char as u8 >= ASCII_0 && char as u8 <= ASCII_9
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
