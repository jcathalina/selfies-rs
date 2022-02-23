use selfies_rust::atom;

fn main() {
    println!("Hello, world!");
    let atom = atom::Atom {
        element: "C".to_string(),
        is_aromatic: false,
        charge: 0,
        config: atom::AtomConfig {
            ..Default::default()
        }
    };
    println!("{:#?}", atom);
}
