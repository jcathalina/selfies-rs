use petgraph::Graph;

use crate::atom::Atom;
use std::collections::HashMap;
use std::vec::Vec;


/// A dataclass that contains a token string and its index.
#[derive(Debug)]
struct Attribute {
    index: i64,
    token: String
}


/// A bond that contains directional information.
#[derive(Debug)]
pub struct DirectedBond {
    pub src: u32,
    pub dest: u32,
    pub order: u32,
    pub stereo: Option<String>,
    pub ring_bond: bool,
}


/// A molecular graph.
/// Molecules can be viewed as weighted undirected graphs. However, SMILES
/// and SELFIES strings are more naturally represented as weighted directed
/// graphs, where the direction of the edges specifies the order of atoms
/// and bonds in the string.
#[derive(Debug)]
struct MolecularGraph {
    roots: Vec<Atom>,
    atoms: Vec<Atom>,
    bond_dict: HashMap<(i32, i32), DirectedBond>,
    adj_list: Graph<i32, i32>,
    bond_counts: Vec<i32>,
    ring_bond_flags: Vec<bool>,
    delocal_subgraph: Graph<i32, i32>,
    attribution: HashMap<Atom, Vec<Attribute>>,
    attributable: bool
}