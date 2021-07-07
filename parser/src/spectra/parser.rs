/**
 * Parses IR spectra .sdf files using nom
 *
 **/

use nom::IResult;

use crate::parser_common::common::*;
use crate::debug_println;
use crate::constants::*;

use super::spectrum::Spectrum;

// Parses through the header until reaching the desired field.
// This assumes that the field is actually in the header.
fn parse_until_field<'a>(i : &'a str, field : &'a str) -> IResult<&'a str, &'a str> {
    let mut out = i; 
    loop {
        // If the next char is numeric, it means this parser has failed to find the field
        // TODO improve imports
        if is_next_tag_x(out, "##XYDATA=") && field != "XYDATA"{
            return Err(nom::Err::Error(nom::error::Error{ input : out, code : nom::error::ErrorKind::Not}));
        }
        // If the next char is not a ##, and not numeric, it should skip to the next line
        if !is_next_char_x(out, '#') {
            out = not_newline(out)?.0;
            continue
        }
        match parse_field(out, field, "##") {
            // This assumes that all errors mean that the field has yet to be found in the header
            // only skips to the next line
            Err(_) => {
                out = not_newline(out)?.0;
            }, Ok(x) => {
                return Ok(x)
            }
        }
    }
}

// Parses until reaching a line which does not start with ##, or reaching the line that starts with
// ##header
// Assumes that the ## has not been parsed yet from i
fn parse_field<'a>(i : &'a str, field : &'a str, escape_str : &'a str) -> IResult<&'a str, &'a str> {
    let mut out : &'a str = i;
    // Removes the "##"
    out = tag_x(out, escape_str)?.0;
    // Parses the field, returns an error otherwise
    out = tag_x(out, field)?.0;
    // Parses the equals sign after the field
    out = char_x(out, '=')?.0; 
    let value = not_newline(out)?;
    Ok(value)
}

// Parses a line of xy data, returning the parsed &str and a vector of the N y values found on the
// current line
// For some reason, some of the jdx files are "+" and "-" delimited, which requires special handling performed by the 
// parse_f32_delimited()
// delimiter param is a list of chars that count as a delimiter
fn parse_xydata_line<'a>(i : &'a str, delimiter : Option<&'a str>) -> IResult<&'a str, Vec<f32>> {
    // Delimiter defaults to " \n" 
    let delimiter : &str = delimiter.unwrap_or(" \n");
    // Parses the inconcequential x value that is at the start of every XYDATA line
    let mut out = parse_f32_delimited(i, delimiter)?.0;
    let mut vec : Vec<f32> = Vec::new();
    loop {
        // Breaks and consumes the "\n" if "\n" is next
        match char_x(out, '\n'){
            Err(_) => {},
            Ok(x) => {
                out = x.0;
                break
            }
        }
        // Parses 1 f32 from out
        let res = parse_f32_delimited(out, delimiter)?;
        out = res.0;
        vec.push(res.1);
    }
    Ok((out, vec))
}

// Returns the value from the field, or the default value
fn get_field_or_default<'a>(i : &'a str, field : &'a str, default : &'a str) -> (&'a str, &'a str) {
    match parse_until_field(i, field) {
        Ok(x) => {
            return (x.0, x.1)
        }, Err(_) => {
            return (i, default)    
        }
    }
}

// Feed this function a jdx filepath and get back a Spectrum struct
pub fn parse_jdx(filepath : &str) -> Result<Spectrum, &str> {
    // Reads to string all at once
    let file : &str;
    let res = &read_file_to_string(filepath);
    match res {
        Err(_) => {
            return Err("Error reading file! Does this file exist?")
        }, 
        Ok(x) => {
            file = x;
        }
    }
    // If this block is run, the file is prpbably not a proper .jdx format
    if !is_next_char_x(file, '#') {
        println!("Error! {} is not a valid .jdx format!", filepath);
        return Err("invalid file")
    }

    // Parses the header to construct the initial Spectrum
    // The order of these statements is important
    let (mut file, mut spectrum) = {
        let (file, title) = get_field_or_default(file, "TITLE", "UNKNOWN TITLE");
        let (file, spectrum_type) = get_field_or_default(file, "DATA TYPE", "UNKNOWN TYPE");
        let (file, state) = get_field_or_default(file, "STATE", "UNKNOWN STATE");
        
        debug_println!("{}, {}, {}", title, spectrum_type, state);

        let (file, y_factor) = get_field_or_default(file, "YFACTOR", "1");
        let y_factor : f32 = y_factor.parse::<f32>().unwrap_or(1.0);
        debug_println!("y_factor {}", y_factor);

        let (file, first_x) = parse_until_field(file, "FIRSTX").unwrap();
        let first_x : f32 = first_x.parse::<f32>().unwrap();
        debug_println!("first_x {}", first_x);

        let (file, last_x) = parse_until_field(file, "LASTX").unwrap();
        let last_x : f32 = last_x.parse::<f32>().unwrap();
        debug_println!("last_x {}", last_x);

        let (file, npoints) = parse_until_field(file, "NPOINTS").unwrap();
        let npoints : i32 = npoints.parse::<i32>().unwrap();
        debug_println!("npoints {}", npoints);

        let mut spec = Spectrum::new(title, spectrum_type, state, first_x, last_x, npoints);
        spec.set_y_factor(y_factor);

        (file, spec)
    };

    // Parses until the XYDATA
    file = parse_until_field(file, "XYDATA").unwrap().0;

    // Finds what delimiter the XYDATA line uses
    // The only possible delimiters are " " and "+"
    let mut delimiter = " \n";
    let not_numeric : &str = not_numeric(file).unwrap().0;
    if is_next_char_x(not_numeric, '+') {
        delimiter = "+\n";
    }
    // Parses until reaching the ##END
    loop {

        // If at the end of the data section
        if is_next_tag_x(file, "##END") {
            break
        }
        let res = parse_xydata_line(file, Some(delimiter));
        match res {
            Err(_) => {
                break
            },
            Ok(x) => {
                file = x.0;
                for val in x.1 {
                    spectrum.add_y(val.to_owned());
                }
            }
        }
    }

    // If the spectrum doesn't have all of the y values alloted for every x value, something has
    // probably gone wrong in the mol file
    assert!(spectrum.is_complete());

    Ok(spectrum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_imports() {
        let res = whitespace("  hello");
        println!("{}", res.unwrap().0);
    }
    #[test]
    fn test_parse_field() {
        let res = parse_field("##SUBJECT NAME=COOL BREEZE\n##MORE STUFF=NULL", "SUBJECT NAME", "##").unwrap();
        println!("{:?},\t {:?}", res.0, res.1);
        assert_eq!(res, ("##MORE STUFF=NULL", "COOL BREEZE"));
    }
    #[test]
    fn test_parse_xydata_line() {
        let res = parse_xydata_line("234.221345 0.4380 0.4380 0.4370 0.4370 0.4269\nTrampled under foot", None);
        let res_vec = res.unwrap().1;
        let should_be = vec!(0.4380, 0.4380, 0.4370, 0.4370, 0.4269);
        for i in 0..should_be.len() {
            assert_eq!(should_be[i], res_vec[i]);
        }
    }
    #[test]
    fn test_parse_delimited_line() {
        let res = parse_xydata_line("-42.1 420.69-18.2-12.1+13.5\nSomething else", None);
        let res_vec = res.unwrap().1;
        // The -42.1 is skipped because parse_xydata_line() skips the first number
        let should_be = vec!(420.69, -18.2, -12.1);
        for i in 0..should_be.len() {
            assert_eq!(should_be[i], res_vec[i]);
        }
    }
    #[test]
    fn test_until_field() {
        let content = &read_file_to_string(&(TEST_DIR.to_string() + "Water.jdx")).unwrap();
        let res = parse_until_field(content, "TITLE").unwrap();
        assert_eq!(res.1, "WATER");
        let res = parse_until_field(content, "MOLFORM").unwrap();
        assert_eq!(res.1, "H2 O");
    }
    #[test]
    fn test_parse_jdx() {
        test_parser("Water.jdx");
        
    }
    #[test]
    fn test_non_utf8() {
        // This file contains non UTF-8
        // let spec = parse_jdx(&(TEST_DIR.to_string() + "sodium chloride.jdx")).unwrap();
        // println!("{}", spec.to_string());
    }
    #[test]
    fn test_funky_file() {
        test_parser("Styrene, oligomers.jdx");
    }
    #[test]
    fn test_whack_files(){
        test_parser("Cumidine.jdx");
        test_parser("Ethane, pentafluoro-.jdx");
    }
    fn test_parser(filepath : &str) -> Spectrum {
        let spec = parse_jdx(&(TEST_DIR.to_string() + filepath)).unwrap();
        println!("{}", spec.to_string());
        spec
    }
}
