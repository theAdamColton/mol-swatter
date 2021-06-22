/**
 * Module for parsing the .mol files using nom parsing
 */


use nom::IResult;

use crate::molecule::molecule::*;

use crate::parser_common::common::*;

use crate::constants::DEBUG_LEVEL;


macro_rules! debug_println {
    ($($args:expr), *) => {
        if DEBUG_LEVEL >= 2 {
            println!($($args), *);
        }
    }
}

// parses the first 4 new lines from i
// returns the number of atoms found on line 4
fn parse_header(i : &str) -> (&str, i32) {
    let mut out = (i, "");
    for _x in 0..3 {
        out= not_newline(out.0).unwrap();
    }
    // gets the number of atoms
    let num_atoms : i32 = parse_i32(out.0).unwrap().1;
    // finished parsing the header
    out = not_newline(out.0).unwrap();
    
    (out.0, num_atoms)
}

// parses the atom section, returning the list of atoms as a vector
fn parse_atom_list(i : &str, num_atoms : i32) -> IResult<&str, Vec<&str>> {
    let mut trimmed = i;
    let mut atoms : Vec<&str> = Vec::new();

    for _i in 0..num_atoms {
        // This section parses the 3d coordinates up the element character
        // For each 3d coordinate
        for _x in 0..3 {
            trimmed = whitespace(trimmed).unwrap().0;
            trimmed = not_whitespace(trimmed).unwrap().0;
        }
        // for the remaining whitespace before the element
        trimmed = whitespace(trimmed).unwrap().0;

        // takes the element char and adds it to the atoms vector
        let to_add = not_whitespace(trimmed).unwrap().1;
        atoms.push(to_add);
        
        // Goes to the next line
        trimmed = not_newline(trimmed)?.0;
        
        debug_println!("{}", trimmed);
    }
    
    Ok((trimmed, atoms))
}

// parses a single line;  single, double, and triple bonds from the mol file format
// Does not parse the stereochemistry section
fn parse_bonds(i : &str) -> IResult<&str, (i32, i32, i32)> {
    let (i, atom1) = parse_i32(i)?;
    let (i, atom2) = parse_i32(i)?;
    let (i, bond_type) = parse_i32(i)?;

    let i = not_newline(i)?.0;

    // The two atoms must be different, (a bond can't be described between an atom and itself)
    assert_ne!(atom1, atom2);

    Ok((i, (atom1, atom2, bond_type)))
}

// Given a valid file path of a .mol file, contructs a Molecule struct from the data in the file
// The main entry to parser
pub fn parse_mol(file_path : &str) -> Molecule {
    let contents = read_file_to_string(file_path).unwrap();
    debug_println!("{}", contents);

    // removes the header, but gets the number of atoms and saves in num_atoms
    let (result, num_atoms) = parse_header(&contents);

    let (mut contents, atoms) = parse_atom_list(result, num_atoms).unwrap();
    let mut molecule : Molecule = Molecule::new(atoms);
    debug_println!("{}", molecule.to_string());

    loop {
        // If the escape character is reached, 'M', then breaks the loop
        // In some mol files, 'A' marks the beggining of a new section after the bond adjacency
        // section
        if is_next_char_x(contents, 'M') || is_next_char_x(contents, 'A') {
            break;
        }

        let result = parse_bonds(contents).unwrap();
        contents = result.0;

        debug_println!("{}", contents);
        
        // the -1 are there because the data in the mol file starts from 1 instead of 0.
        // the bond doesnt need to be set to -1 because the default is 0
        molecule.add_bond((result.1.0 - 1) as usize, (result.1.1 - 1) as usize, result.1.2);
    }

    debug_println!("{}", molecule.to_string());

    molecule
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::TEST_DIR;
    #[test]
    fn test_read_file() {
        println!("{}", read_file_to_string(&(TEST_DIR.to_owned() + "Pentanoic acid.mol")).unwrap());
    }
    #[test]
    fn test_mol_parse() {
        let molecule = get_mol("Pentanoic acid.mol");
        println!("{}", molecule.to_string());
    }
    #[test]
    fn test_debug_print() {
        debug_println!("Hello");
    }
    #[test]
    fn test_imports() {
        let molecule : Molecule = Molecule::new(vec!["C", "O", "C", "K"]);
        println!("{}", molecule.to_string());
    }
    #[test]
    fn test_tricky_mol() {
        let molecule = get_mol("Decaborane.mol");
        println!("{}", molecule.to_string());
        let molecule = get_mol("1,2-Benzenedicarboxylic acid, diisooctyl ester.mol");
        println!("{}", molecule.to_string());
    }
    fn get_mol(file : &str) -> Molecule {
        parse_mol(&(TEST_DIR.to_owned() + file))
    }
}
