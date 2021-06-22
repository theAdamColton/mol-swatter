/*
Traverses the molecule adjacency matrix and recognizes against the functional group patterns
*/

use crate::molecule::molecule::*;

static DEBUG_LEVEL : i32 = 1;

macro_rules! debug_println {
    ($($args:expr), *) => {
        if DEBUG_LEVEL >= 2 {
            println!($($args), *);
        }
    }
}


// finds sub instances of the subMole in majorMol
// goes through each atom of the majorMol, until it exhausts the search or finds the whole
// subgraph of subMol in majorMol
// Matches atom and bond arrangements\
// Usually, the least common organic atoms are at the end of the list, so it would be more
// efficient to start from the end of the atom vec
// TODO Needs to be optimized.
pub fn is_subgraph(maj_mol : &Molecule, sub_mol : &Molecule) -> bool {

    debug_println!("{}", maj_mol.to_string());
    debug_println!("{}", sub_mol.to_string());
  
    let maj_atoms = &maj_mol.atoms;
    let sub_atoms = &sub_mol.atoms;

    // sub_atom is the first atom in the sub_mol that is not R
    let sub_atom_j = get_not_r(sub_atoms);
    let sub_atom = &sub_atoms[sub_atom_j];

    for i in (0..maj_atoms.len()).rev() {
        let maj_atom = &maj_atoms[i];
        if maj_atom == sub_atom {

            // visited keeps track of the visited sub atoms
            let mut sub_visited = vec![false; sub_atoms.len()];
            let mut maj_visited = vec![false; maj_atoms.len()];
            
            let res = is_subgraph_from_here(maj_mol, i, sub_mol, sub_atom_j ,
                &mut sub_visited, &mut maj_visited);

            if res {return true}
        }
    }
    false
} 

fn get_not_r(atoms : &Vec<String>) -> usize {
    for j in (0..atoms.len()).rev() {
        if atoms[j] != "R" {
            return j     
        }
    }
    return 0
}

fn is_subgraph_from_here(maj_mol : &Molecule, maj_i : usize,
    sub_mol : &Molecule, sub_i : usize, visited_sub_atoms : &mut Vec<bool>,
    visited_maj_atoms : &mut Vec<bool>) -> bool {

    debug_println!("is_subgraph_from_here(maj_i : {}, sub_i : {})", maj_i, sub_i);

    // If cyclic
    if visited_sub_atoms[sub_i] {
        debug_println!("true, sub visited");
        return true 
    }
    if visited_maj_atoms[maj_i] {
        debug_println!("false, maj visited");
        return false 
    }

    // marks the current sub_atom as visited
    visited_sub_atoms[sub_i] = true;
    visited_maj_atoms[maj_i] = true;

    let maj_bonds = get_bond_list(maj_mol, maj_i);
    let sub_bonds = get_bond_list(sub_mol, sub_i);

    for j in 0..sub_bonds.len() {
        if sub_bonds[j] == 0 {
            continue
        }
        
        // Allows "R" to act as H atom, meaning nothing - which means no maj_bonds apart from 0
        let mut should_check_maj_bonds = false;
        let mut cntr = 0;
        if sub_mol.atoms[j] == "R" {
            should_check_maj_bonds = true; 
        }
        
        let mut at_least_one = false;
        for i in 0..maj_bonds.len() {
            if maj_bonds[i] == 0 {
                continue
            }

            // Counts the number of maj_bonds
            if should_check_maj_bonds {
                cntr += 1;
            }

            if sub_bonds[j] != maj_bonds[i] {
                continue
            }

            if sub_mol.atoms[j] == "R" && maj_mol.atoms[i] == "C" {
                at_least_one = true;
                break
            }

            // sub_mol.atoms[j] refers to the recieving atom of the current bond that is being 
            // examined.
            if sub_mol.atoms[j] != maj_mol.atoms[i] {
                continue
            }

            at_least_one = is_subgraph_from_here(maj_mol, i, sub_mol, j, visited_sub_atoms, 
                visited_maj_atoms);
            if at_least_one {break}
        }
        
        // handles 0 maj_bonds    (can still count as being bonded to hydrogen)
        if should_check_maj_bonds && cntr == 0 {
            continue
        }

        if !at_least_one {debug_println!("false"); return false}
    }
    debug_println!("true");
    true
}


// Returns a vec of the bond list
fn get_bond_list(mol : &Molecule, index : usize) -> Vec<i32> {
    let len = mol.atoms.len();
    let mut out : Vec<i32> = Vec::new();
    for x in 0..len {
        out.push(mol.get(index, x));
    }
    out
}


// For tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::funct_groups::funct_groups::aryl;
    use crate::funct_groups::funct_groups::carboxyl;
    use crate::funct_groups::funct_groups::ether;
    use crate::molecule::parser::parse_mol;
    use crate::funct_groups::funct_groups::carbonyl;

    #[test]
    fn demonstration() {
        assert!(is_subgraph(&parse_mol("src/molecule/test_files/Pentanoic acid.mol"), &carbonyl().molecule));
    }
    #[test]
    fn test_is_subgraph() {
        let maj_mol = &parse_mol("src/molecule/test_files/Pentanoic acid.mol");
        let sub_mol = &carbonyl().molecule;
        let mut res = is_subgraph(maj_mol, sub_mol);
        assert!(res);
        let sub_mol = &carboxyl().molecule;
        println!("{}", sub_mol.to_string());
        res = is_subgraph(maj_mol, sub_mol); 
        assert!(res);

        let maj_mol = &parse_mol("src/molecule/test_files/Benzoic acid.mol");
        let sub_mol = &carbonyl().molecule;
        res = is_subgraph(maj_mol, sub_mol);
        assert!(res);
        let sub_mol = &carboxyl().molecule;
        res = is_subgraph(maj_mol, sub_mol);
        assert!(res);

        let maj_mol = &parse_mol("src/molecule/test_files/Methane.mol");
        assert_eq!(is_subgraph(maj_mol, sub_mol), false);
        let sub_mol = &carboxyl().molecule;
        assert_eq!(is_subgraph(maj_mol, sub_mol), false);
        let sub_mol = &aryl().molecule;
        assert_eq!(is_subgraph(maj_mol, sub_mol), false);
    }
    #[test]
    fn test_r_wildcard() {
        let maj_mol = &parse_mol("src/molecule/test_files/Benzoic acid.mol");
        let sub_mol = &ether().molecule;
        assert_eq!(is_subgraph(maj_mol, sub_mol), false);
    }
    #[test]
    fn test_debug_print() {
        debug_println!("Hello {} {} {}", 1, 2, 3)
    }
    #[test]
    fn test_cyclic() {
        let maj_mol = &parse_mol("src/molecule/test_files/Benzoic acid.mol");
        println!("{}", maj_mol.to_string());
        assert!(is_subgraph(maj_mol, &carboxyl().molecule));
        assert!(is_subgraph(maj_mol, &aryl().molecule))
    }
}
