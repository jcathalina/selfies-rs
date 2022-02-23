use std::ops::{Deref, DerefMut};


/// The base definition of an atom with mandatory specifications.
#[derive(Debug)]
pub struct Atom {
    pub element: String,
    pub is_aromatic: bool,
    pub charge: i8,
    pub config: AtomConfig,
}

/// Optional additional specifications for the definition of an atom (e.g., charge, chirality).
#[derive(Debug)]
pub struct AtomConfig {
    pub isotope: Option<i32>,
    pub chirality: Option<String>,
    pub h_count: Option<i32>,
}

impl Deref for Atom {
    type Target = AtomConfig;
    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for Atom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

impl Default for AtomConfig {
    fn default() -> Self {
        AtomConfig {
            isotope: None,
            chirality: None,
            h_count: None,
        }
    }
}

impl Atom {
    /// Inverts the chirality of the current atom from @ to @@, and vice versa.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use selfies_rust::atom::{Atom, AtomConfig};
    /// let mut atom = Atom {
    ///    element: "C".to_string(),
    ///    is_aromatic: false,
    ///    charge: 0,
    ///    config: AtomConfig {
    ///    chirality: Some("@".to_string()),
    ///    ..Default::default()
    ///    }
    /// };
    /// atom.invert_chirality();
    /// assert_eq!(atom.chirality.as_ref(), Some("@@".to_string()).as_ref());
    /// ```
    pub fn invert_chirality(&mut self) -> () {
        if self.chirality == Some("@".to_string()) {
            self.chirality = Some("@@".to_string());
        }
        else if self.chirality == Some("@@".to_string()) {
            self.chirality = Some("@".to_string());
        }
    }
}