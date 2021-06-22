// Structs related to instatiating and modifying molecules, represented by 
// a symmetrical edge matrix representing bonds between atoms.
pub struct Molecule {
    // the edge matrix
    matrix : Matrix,
    // the vector key storing the atom types
    pub atoms : Vec<String>,
}


impl Molecule {
    // Instantiate a new Molecule
    pub fn new(atoms : Vec<&str>) -> Molecule {
        let atoms = {
            let mut out:Vec<String> = Vec::new();
            for element in atoms {
                out.push(String::from(element));
            }
            out
        };

        Molecule {
            matrix : Matrix::new(atoms.len() as usize),
            atoms
        }
    }

    // Add a single bond between two atoms in the molecule
    // returns a happy string if the new bond was valid and recorded,
    // returns an error string if the new bond was invalid, (in an occupied location)
    pub fn add_bond(&mut self, x : usize, y : usize, z : i32) {
        if self.matrix.get(x as usize, y as usize) != 0 {
            panic!("{}", "bond has already been set!")
        } else {
            self.matrix.set(x, y, z);
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for a in &self.atoms {
            out += &a.to_string();
        }
        out += "\n";
        out += &self.matrix.to_string();
        out
    }

    // Get a bond from the matrix
    pub fn get(&self, x : usize, y : usize) -> i32 {
        self.matrix.get(x as usize, y as usize)
    }

    // Returns the matrix
    pub fn get_matrix(&self) -> &Vec<Vec<i32>> {
        &self.matrix.matrix
    }
}

struct Matrix {
    matrix : Vec<Vec<i32>>,
}
impl Matrix {
    // initializes an empty 2d array. 
    pub fn new(size : usize) -> Matrix{
        Matrix {matrix : vec![vec![0; size as usize]; size] }
    }

    // set a spot in the matrix (add a new bond between atoms)
    // since this represents a unidirectional graph, the matrix has to be symmetrical
    pub fn set(&mut self, x: usize, y: usize, amount: i32 ) {
        self.matrix[x][y] = amount;
        self.matrix[y][x] = amount;
    }

    pub fn get(&self, x: usize, y : usize) -> i32 {
        self.matrix[x][y]
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for x in 0..self.matrix.len() {
            for y in 0..self.matrix[x].len() {
                out += &self.matrix[x][y].to_string();
            }
            out += "\n"
        }
        out
    }
}



// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecule_constructor() {
        let mut my_mol = Molecule::new(vec![
            "C", "O", "C", "K"]);

        println!("{}", my_mol.to_string());

        // adds a double bond between 'C' and 'O'
        let _result = my_mol.add_bond(1, 2, 2);

        // attempting to add another bond will result in an error
        //let _result = my_mol.add_bond(1, 2, 1);
    }
    #[test]
    fn test_add_bond() {
        let mut my_mol = Molecule::new(vec![
            "C", "O", "C","K"]);
        println!("{}", my_mol.to_string());
        my_mol.add_bond(1, 2, 1);
        println!("{}", my_mol.to_string());
        my_mol.add_bond(3, 2, 1);
        println!("{}", my_mol.to_string());
        my_mol.add_bond(3, 1, 2);
        println!("{}", my_mol.to_string());
    }
    #[test]
    fn test_matrix_methods() {
        let mut matrix = Matrix::new(10);
        println!("{}", matrix.to_string());

        matrix.set(1,5,1);

        println!("{}", matrix.to_string());

        assert_eq!(matrix.matrix[1][5], 1);
        assert_eq!(matrix.matrix[5][1], 1);
    }
}
