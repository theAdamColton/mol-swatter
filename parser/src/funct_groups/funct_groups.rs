/**
 * Contains a list of all of the functional groups used to create the funct group vector
 *
 * Functional groups are defined as a custom struct, structural data is 
 *  defined as a molecule
 *
 **/

use crate::molecule::molecule::*;

// contains all the data required to express a functional group,
// stored as an Molecule subset
// compared with other Molecules as a subgraph
pub struct FunctGroup {
    pub name : &'static str,
    pub molecule: Molecule,
}
impl std::fmt::Display for FunctGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}


// returns a vec of all the functional groups 
// calls all of the molecule contructors so might be slightly expensive
// YOU NEED TO PUSH THE NEW FUNCTIONAL GROUP TO THIS AFTER DEFINING IT :)
pub fn get_funct_groups() -> Vec<FunctGroup> {
    let mut o : Vec<FunctGroup> = Vec::new();
    
    o.push(carbonyl()); o.push(carboxyl()); o.push(acid_chloride());
    o.push(aryl()); o.push(hydroxyl()); o.push(ester());
    o.push(peroxide()); o.push(ether()); o.push(anhydride());
    o.push(ether()); o.push(anhydride()); o.push(amide());
    o.push(amine()); o.push(imine()); o.push(nitrile());
    o.push(pyridyl()); o.push(nitro()); o.push(sulfide());
    o.push(disulfide()); o.push(sulfide()); o.push(sulfoxide());
    o.push(phosphonic_acid()); o.push(phosphate());
    o.push(borino()); o.push(borono()); o.push(boronate());
    o.push(RLi()); o.push(grignard());

    o
}

/* Remember that "R" stands for a bond to hydrogen or carbon or nothing!
 *
 * Thus, a carboxylic acid will contain ROR,
 * to signify an ether : COC  is correct, ROR could mean a hydroxide or even water!
 *
 */

// *************** Aryl ************************

pub fn aryl() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("C", "C", "C", "C", "C", "C"));
    molecule.add_bond(0, 1, 2);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(2, 3, 2);
    molecule.add_bond(3, 4, 1);
    molecule.add_bond(4, 5, 2);
    molecule.add_bond(5, 0, 1);
    FunctGroup { molecule, name : "Aryl"}
}

// ************* oxygen containing *************

// any sp2 carbon bonded to a neutral oxygen
// (the oxygen  must not be bonded to any other atoms)
pub fn carbonyl() -> FunctGroup {
    // initializes a new molecule with only C and O
    let mut carboxyl = Molecule::new(vec!("C", "O"));
    // adds a double bond between C and O
    carboxyl.add_bond(0, 1, 2);
    // returns a functional group made from the carboxyl molecule
    FunctGroup {
        molecule : carboxyl,
        name : "Carbonyl"
    }
}

pub fn carboxyl() -> FunctGroup {
    let mut mol = Molecule::new(vec!("C", "O", "O"));
    mol.add_bond(0, 1, 2);
    mol.add_bond(0, 2, 1);
    FunctGroup {
        molecule : mol,
        name : "Carboxyl"
    }
}

pub fn acid_chloride() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("C", "O", "Cl"));
    molecule.add_bond(0, 1, 2);
    molecule.add_bond(0, 2, 1);
    FunctGroup { molecule, name : "Acid Chloride" }
}



pub fn hydroxyl() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("C", "O"));
    molecule.add_bond(0, 1, 1);
    FunctGroup { molecule, name : "Hydroxyl" }
}

pub fn ester() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("C", "O", "O", "C"));
    molecule.add_bond(0, 1, 2);
    molecule.add_bond(0, 2, 1);
    molecule.add_bond(2, 3, 1);
    FunctGroup { molecule, name : "Ester" }
}

pub fn peroxide() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "O", "O", "R"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(2, 3, 1);
    FunctGroup {molecule, name : "Peroxide"}
}

pub fn ether() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("C", "O", "C"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    FunctGroup { molecule , name : "Ether"}
}

pub fn anhydride() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "C", "O", "O", "C", "O", "R"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 2);
    molecule.add_bond(1, 3, 1);
    molecule.add_bond(3, 4, 1);
    molecule.add_bond(4, 5, 2);
    molecule.add_bond(4, 6, 1);
    FunctGroup {molecule, name : "Anhydride"}
}

// ***************** nitrogen containing ***********************

pub fn amide() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "C", "O", "N", "R", "R"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 2);
    molecule.add_bond(1, 3, 1);
    molecule.add_bond(3, 4, 1);
    molecule.add_bond(3, 5, 1);
    FunctGroup {molecule, name : "Amide"}
}
pub fn amine() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("N", "R"));
    molecule.add_bond(0, 1, 1);
    FunctGroup{molecule, name : "Amine"}
}
// TODO this only recognizes a nitrogen doublebonded to a carbon, doesnt specify what the other
// group bonded to the nitrogen is
pub fn imine() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("N", "R"));
    molecule.add_bond(0, 1, 2);
    FunctGroup{molecule, name : "Imine"}
}
pub fn nitrile() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "C", "N"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 3);
    FunctGroup{molecule, name : "Nitrile"}
}
pub fn pyridyl() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("C", "C", "C", "C", "C", "N"));
    molecule.add_bond(0, 1, 2);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(3, 4, 2);
    molecule.add_bond(4, 5, 1);
    molecule.add_bond(5, 0, 2);
    FunctGroup{molecule, name : "Pyridyl"}
}
pub fn nitro() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "N", "O", "O"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 2);
    molecule.add_bond(2, 3, 1);
    FunctGroup{molecule, name : "Nitro"}
}

// ************* sulfur containing *****************

pub fn sulfide() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "S", "R"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    FunctGroup{molecule, name : "Sulfide"}
}
pub fn disulfide() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "S", "S", "R"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(2, 3, 1);
    FunctGroup{molecule, name : "Disulfide"}
}
pub fn sulfoxide() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "S", "O", "R"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 2);
    molecule.add_bond(2, 3, 1);
    FunctGroup{molecule, name : "Sulfoxide"}
}

// ************ phosphorus containing **************

pub fn phosphonic_acid() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "P", "O", "O", "O"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 2);
    molecule.add_bond(1, 3, 1);
    molecule.add_bond(1, 4, 1);
    FunctGroup{molecule, name : "Phosphonic Acid"}
}
pub fn phosphate() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "O", "P", "O", "O", "O"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(2, 3, 2);
    molecule.add_bond(2, 4, 1);
    molecule.add_bond(2, 5, 1);
    FunctGroup{molecule, name : "Phosphate"}
}

// ****************** boron containing ***************************

pub fn borono() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "B", "O", "O"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(2, 3, 1);
    FunctGroup{molecule, name : "Borono"}
}
pub fn boronate() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "B", "R", "O"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(1, 3, 1);
    FunctGroup{molecule, name : "Boronate"}
}
pub fn borino() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "B", "R", "O"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    molecule.add_bond(1, 3, 1);
    FunctGroup{molecule, name : "Borino"}
}

// ******************* organo metallics ****************************8

pub fn RLi() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "Li"));
    molecule.add_bond(0, 1, 1);
    FunctGroup{molecule, name : "RLi"}
}
pub fn grignard() -> FunctGroup {
    let mut molecule = Molecule::new(vec!("R", "Mg", "X"));
    molecule.add_bond(0, 1, 1);
    molecule.add_bond(1, 2, 1);
    FunctGroup{molecule, name : "Grignard"}
}
