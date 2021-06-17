/*
Main entry for the C foreign-function interface
*/

#[macro_use] extern crate nom;

mod molecule;
mod funct_groups;
mod get_res;
mod spectra;
mod parser_common;
mod constants;

use molecule::molecule::*;
use molecule::parser::*;
use get_res::{GetRes, FunctGroupResult};

use spectra::{spectrum, parser::parse_jdx};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Object for parsing functional groups from .mol files
#[pyclass]
struct ParseGroups {
    GetRes : GetRes,
}

#[pymethods]
impl ParseGroups {

    // If no method marked with #[new] is declared, object instances 
    // can only be created from Rust, but not from Python.
    #[new]
    fn new() -> Self {
       ParseGroups{GetRes : GetRes::new()}
    }
    fn get_funct_result(&mut self, file_path :&str) -> PyResult<Vec<bool>> {
       Ok(self.GetRes.get_res_from_file(file_path).result)
    }

    fn get_funct_result_and_print(&mut self, file_path : &str) -> PyResult<Vec<bool>> {
        Ok(self.GetRes.get_res_from_file_and_print(file_path).result)
    }

    fn get_funct_groups(&mut self) -> PyResult<Vec<String>> {
        Ok(self.GetRes.get_funct_groups())
    }

    fn get_matrix(&self, file_path : &str) -> PyResult<Vec<Vec<i32>>> {
        Ok(self.GetRes.get_matrix(file_path))
    }
}

// Object for parsing and transforming 2d data from .jdx spectra
#[pyclass]
struct Spectrum {
    spectrum : Option<spectrum::Spectrum>,
}
#[pymethods]
impl Spectrum {
    #[new]
    fn new(filepath : &str) -> Self {
        match parse_jdx(filepath) {
            Ok(x) => {
                return Spectrum{spectrum : Some(x)}
            }, Err(_) => {
                return Spectrum{spectrum : None}
            }
        }
    }
    fn is_valid(&self) -> bool {
        if self.spectrum.is_none()  {
            return false
        } else {
            return true
        }
    }
    // Returns a new tranformed Spectrum
    fn transform(&self, first_x : f32, last_x : f32, npoints : i32) -> Spectrum {
        Spectrum{spectrum : Some(self.spectrum.as_ref().unwrap().transform(first_x, last_x, npoints))}
    }
    fn get_x_values(&self) -> Vec<f32> {
        self.spectrum.as_ref().unwrap().get_x_values()
    }
    fn get_y_values(&self) -> Vec<f32> {
        self.spectrum.as_ref().unwrap().get_y_values()
    }
    fn to_string(&self) -> String {
        self.spectrum.as_ref().unwrap().to_string()
    }
}

// Defines the mol_swatter python module
#[pymodule]
fn mol_swatter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParseGroups>()?;
    m.add_class::<Spectrum>()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_imports() {
        let mut molecule : Molecule = Molecule::new(vec!["a", "b", "c", "d"]);
        parse_mol("src/molecule/test_files/Pentanoic acid.mol");
    }
}
