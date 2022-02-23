use rstest::*;
use selfies_rust::atom;

#[rstest]
fn test_chirality_is_inverted_successfully() {
    let mut atom = atom::Atom {
        element: "C".to_string(),
        is_aromatic: false,
        charge: 0,
        config: atom::AtomConfig {
            chirality: Some("@".to_string()),
            ..Default::default()
        }
    };
    println!("{:#?}", atom);
    atom.invert_chirality();
    println!("{:#?}", atom);
    assert_eq!(atom.chirality.as_ref(), Some("@@".to_string()).as_ref());
}
