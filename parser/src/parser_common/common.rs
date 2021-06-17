/**
 * Common functions used for the .mol molecule parser and the .jdx ir_spectra parser
 *
 * Check the tests for use cases
 **/

use nom::{
    IResult, bytes::complete::{
        is_not,
        tag,
        is_a,
        take_while,
    }, 
    character::{
        complete::{
            char, 
            digit1,
            one_of,
        },
        is_digit,
    },
    combinator::peek,
    error::ErrorKind,
    branch::alt,
};

use std::fs;

use crate::constants::DEBUG_LEVEL;

#[macro_export]
macro_rules! debug_println {
    ($($args:expr), *) => {
        if DEBUG_LEVEL >= 2 {
            println!($($args), *);
        }
    }
}


// parses from i until reaches " " or "\t"
pub fn not_whitespace(i : &str) -> IResult<&str, &str> {
    is_not(" \t\n")(i)
}

// parses from i until reaching x
pub fn not_x<'a>(i : &'a str, x : &'a str) -> IResult<&'a str, &'a str> {
    is_not(x)(i)
}

// parses whitespace, until it reaches not whitespace
pub fn whitespace(i : &str) -> IResult<&str, &str> {
    is_a(" \t")(i)
}

// Parses from i until reaches "\n", parsing the "/n"
pub fn not_newline(i : &str) -> IResult<&str, &str> {
    let result = is_not("\n")(i)?;
    let next_line = tag("\n")(result.0)?.0;
    Ok((next_line, result.1))
}

pub fn char_x(i : &str, x : char) -> IResult<&str, char> {
    char(x)(i)
}

pub fn tag_x<'a>(i : &'a str, x : &'a str) -> IResult<&'a str, &'a str> {
    tag(x)(i)
}

pub fn is_next_tag_x(i : &str, x : &str) -> bool {
    match tag_x(i, x) {
        Ok(_) => { return true },
        Err(_) => { return false }
    }
}

pub fn peek_digit(i : &str) -> IResult<&str, &str> {
    peek(digit1)(i)
}

// Returns an false if at least one next char is not numberic
pub fn is_next_char_numeric(i : &str) -> bool {
    match peek_digit(i) {
        Ok(_) => {
            return true
        },
        Err(_) => {
            return false
        }
    }
}

// Returns true if chr is a digit or .
fn is_char_digit(chr: char) -> bool {
    return chr.is_ascii() && (is_digit(chr as u8) || chr == '.')
}

// Parses until the first non numberic char
pub fn not_numeric(i : &str) -> IResult<&str, &str> {
    take_while(is_char_digit)(i)
}

// peeks the next char, sees if it is x
pub fn is_next_char_x(i : &str, x : char) -> bool {
    match char_x (i, x) {
        Err(_) => {
            return false
        },
        Ok(_) => {
            return true
        }
    }
}

// munches through whitespace to find the first non whitespace character, and parses it as an int
// until the next whitespace
pub fn parse_i32(i: &str) -> IResult<&str, i32> {
    let mut trimmed = i;
    // will munch whitespace before the phrase if whitespace exists
    match whitespace(i) {
        Err(_) => {},
        Ok(x) => {
            trimmed = x.0;
        }
    }
    // parses until the next whitespace
    let result = not_whitespace(trimmed)?;
    // returns the result of parsing the bytes as an int
    Ok((result.0, result.1.parse::<i32>().unwrap()))
}


// Very similar to parse_i32, only parses a float instead
pub fn parse_f32(i : &str) -> IResult<&str, f32> {
    let mut i = i;
    match whitespace(i) {
        Err(_) => {},
        Ok(x) => {
            i = x.0;
        }
    }
    let result = not_whitespace(i)?;
    Ok((result.0, result.1.parse::<f32>().unwrap()))
}

// Pass a string to serve as the delimiter
// The delimiter and the - sign can't both be before the f32, only either or
// EX:
// input: "-123.3-23.41 12.43", " " -> output: "-23.41 12.43", -123.3
// 
pub fn parse_f32_delimited<'a>(i : &'a str, delimiter : &'a str) -> IResult<&'a str, f32> {
    let mut i = i;
    // Parses the delimiter from the start of the str if the delimiter is there
    match is_a::<_, _, (&str, ErrorKind)>(delimiter)(i) {
        Err(_) => {},
        Ok(x) => {
            i = x.0;
        }
    }
    let mut neg : bool = false;
    // Handles the + or - signs
    match char_x(i, '+') {
        Err(_) => {},
        Ok(x) => {i = x.0}
    }
    match char_x(i, '-') {
        Err(_) => {},
        Ok(x) => {
            i = x.0;
            neg = true;
        }
    }
    // Parses until the first non 0-9 or '.'
    let res = not_numeric(i)?;
    Ok((res.0, 
            // Multiplies by -1 if neg
            {res.1.parse::<f32>().unwrap() * {if neg {-1.0} else {1.0}}}
            ))
}

// reads the contents of a file to a string, panics if something goes wrong
pub fn read_file_to_string(file_path : &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_DIR;

    #[test]
    fn test_not_whitespace() {
        assert_eq!(not_whitespace("parsesUpToTheFirstSpace here"), Ok((" here", "parsesUpToTheFirstSpace")) );
        let mut and_str = "Hello there";
        match not_whitespace(and_str) {
            Err(_) => {},
            Ok(x) => {
                and_str = x.0;
            }
        }
        println!("{}", and_str);
    }
    #[test]
    fn test_not_x() {
        assert_eq!(not_x("parses up to the first x +here", "+3123"), Ok(("+here", "parses up to the first x ")));
    }
    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   here"), Ok(("here", "   ")));
        let mut and_str = " Hello there";
        match whitespace(and_str) {
            Err(_) => {},
            Ok(x) => {
                and_str = x.0;
            }
        }
        println!("{}", and_str);
    }
    #[test]
    fn test_not_newline() {
        assert_eq!(not_newline("parses up to the first newline\nhere"), Ok(("here", "parses up to the first newline")));
    }
    #[test]
    fn test_tag_x() {
        let res  = tag_x("Prague", "Jason Bourne");
        assert!(res.is_err());
        let res = tag_x("harry potter", "harry ");
        assert_eq!(res.unwrap().0, "potter");
        
    }
    #[test]
    fn test_is_next_char_numeric() {
        let result = is_next_char_numeric("1Hello");
        assert!(result);
        let result = is_next_char_numeric("1111fhdskafh");
        assert!(result);
        let result = is_next_char_numeric("hello");
        assert_eq!(result, false);
    }
    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("   1  "), Ok(("  ", 1)));
        assert_eq!(parse_i32("12312   "), Ok(("   ", 12312)));
    }
    #[test]
    fn test_parse_f32() {
        assert_eq!(parse_f32("        3.14  "), Ok(("  ", 3.14)));
        assert_eq!(parse_f32("6.022 avacado"), Ok((" avacado", 6.022)));
        assert_eq!(parse_f32(" 870992122"), Ok(("", 870992122.0)));
    }
    #[test]
    fn test_parse_f32_delimited() {
        assert_eq!(parse_f32_delimited("o420.69o2", "o"), Ok(("o2", 420.69)));
        assert_eq!(parse_f32_delimited("+870976752+\nMore stuff", "+"), Ok(("+\nMore stuff", 870976752.0)));
        assert_eq!(parse_f32_delimited("+42\nMore stuff", "+\n"), Ok(("\nMore stuff", 42.0)));
        assert_eq!(parse_f32_delimited("42-20.1-10.0", "-\n"), Ok(("-20.1-10.0", 42.0)));
        assert_eq!(parse_f32_delimited("-20.1-10.0\n", "\n"), Ok(("-10.0\n", -20.1)));
    }
    #[test]
    fn test_is_next_char() {
        assert!(is_next_char_x("jason bourne", 'j'));
        assert_eq!(is_next_char_x("figero_hello", 'c'), false);
    }
    #[test]
    fn test_read_file() {
        println!("{}", read_file_to_string(&(TEST_DIR.to_owned() + "Pentanoic acid.mol")).unwrap());
    }
    #[test]
    fn test_not_numeric() {
        assert_eq!(not_numeric("420.69here"), Ok(("here", "420.69")));
    }
} 
