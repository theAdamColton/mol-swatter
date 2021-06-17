/*
This module exposes a stuct FunctGetRes for easy result getting.
For examples on how to use the FunctGetRes check out the tests
*/


use super::funct_groups::recognizer;
use super::funct_groups::funct_groups::*;
use super::Molecule;
use crate::molecule::parser::*;


// Stores the result of a calculation. Includes the vec of functional groups tested on
pub struct FunctGroupResult <'a> {
    pub functional_groups : &'a Vec<FunctGroup>,
    pub result : Vec<bool>
}
// Display implementation for printing
impl std::fmt::Display for FunctGroupResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();
        out += "[";
        for i in 0..self.functional_groups.len() {
            out += self.functional_groups[i].name;
            out += " : ";
            out += &self.result[i].to_string(); // sorry
            out += ", ";
        }
        out += "]\n";
        write!(f, "{}", out)
    }
}

// stores data for the getter, so it doesnt have to calculate the functional groups every time.
pub struct GetRes {
    pub functional_groups : Option<Vec<FunctGroup>>,
}

impl<'a> GetRes { 
    pub fn new() -> GetRes {
        GetRes { functional_groups : None}
    }

    pub fn get_funct_groups(&mut self) -> Vec<String> {
        self.gen_funct_groups();
        let mut out : Vec<String> = Vec::new();
        for funct_group in self.functional_groups.as_ref().unwrap() {
            out.push(String::from(funct_group.name));
        }
        out
    }

    // pass a file get the result
    pub fn get_res_from_file(&mut self, mol_file : &str) -> FunctGroupResult {
        let mol : &Molecule = &parse_mol(mol_file);
        self.get_res(mol)
    }

    // gets the result and prints the output
    pub fn get_res_from_file_and_print(&mut self, mol_file : &str) -> FunctGroupResult {
        let res = self.get_res_from_file(mol_file);
        println!("{}", res);
        res
    }

    // Pass a Molecule to get the result
    pub fn get_res(&mut self, mol : &Molecule) -> FunctGroupResult {
        // generates the functional groups if necessary
        self.gen_funct_groups();

        let mut res : Vec<bool> = Vec::new();
        for group in self.functional_groups.as_ref().unwrap() {
            res.push(recognizer::is_subgraph(mol, &group.molecule));
        }

        FunctGroupResult{
            functional_groups : &self.functional_groups.as_ref().unwrap(),
            result : res
        }
    }

    // Pass a .mol file and get a Vec<Vec(i32)>> representation of the adjacency graph
    pub fn get_matrix(&self, file_path : &str) -> Vec<Vec<i32>> {
        let mol : &Molecule = &parse_mol(file_path);
        mol.get_matrix().to_vec()
    }

    fn gen_funct_groups(&mut self) {
        if self.functional_groups.is_none() {
            self.functional_groups = Some(get_funct_groups());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::molecule::parser;
    #[test]
    fn test_get_funct_groups() {
        get_funct_groups();
    }
    #[test]
    fn test_funct_get_res() {
        // Demonstration of FunctGetRes instatiation and result getting
        let mol = &parser::parse_mol("src/molecule/test_files/Benzoic acid.mol");
        let mut res_getter = GetRes::new();
        let res = res_getter.get_res(mol);
        println!("Benzoic acid : {}", res);
    }
    #[test]
    fn test_get_res_from_file() {
        let mut res_getter = GetRes::new();
        let res = res_getter.get_res_from_file("src/molecule/test_files/Methane.mol");
        println!("Methane : {}", res);
    }
    #[test]
    fn test_get_matrix() {
        let res_getter = GetRes::new();
        let res = res_getter.get_matrix("src/molecule/test_files/Methane.mol");
        for x in 0..res.len() {
            for y in 0..res[x].len() {
                print!("{}, ", res[x][y]);            
            }
            println!("");
        }
    }
}
