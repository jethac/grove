//! Grove - Procedural tree generation library
//!
//! A standalone procedural tree generation tool for real-time game engines.
//! Generates 3D tree meshes with LOD, wind animation data, and AI-generated textures.
//!
//! # Example
//!
//! ```
//! use grove_core::{Species, generate_tree};
//!
//! let toml = r#"
//! [species]
//! name = "Oak"
//!
//! [trunk]
//! height = 6.0
//! radius = 0.45
//!
//! [branches.level1]
//! count = 5
//! length = 3.0
//! "#;
//!
//! let species = Species::from_toml(toml).unwrap();
//! let tree = generate_tree(&species, 12345);
//!
//! println!("Generated {} stems", tree.stem_count());
//! ```

pub mod constants;
pub mod generation;
pub mod math;
pub mod rng;
pub mod species;
pub mod tree;

pub use constants::*;
pub use generation::generate_tree;
pub use rng::Rng;
pub use species::Species;
pub use tree::{BoundingBox, Leaf, Segment, Stem, Tree};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_generation() {
        let toml = r#"
[species]
name = "Test Oak"

[trunk]
height = 6.0
radius = 0.45
segments = 6

[branches.level1]
count = 5
length = 3.0
segments = 4

[crown]
shape = "spherical"
"#;

        let species = Species::from_toml(toml).unwrap();
        let tree = generate_tree(&species, 42);

        // Verify basic tree structure
        assert_eq!(tree.species_name, "Test Oak");
        assert_eq!(tree.seed, 42);
        assert!(!tree.stems.is_empty());

        // Verify trunk exists
        let trunk = tree.trunk().unwrap();
        assert_eq!(trunk.level, 0);
        assert!(!trunk.segments.is_empty());

        // Verify branches were generated
        let branch_count = tree.stems_at_level(1).count();
        assert!(branch_count > 0);

        // Verify bounds are valid
        assert!(tree.bounds.is_valid());
    }

    #[test]
    fn test_deterministic_generation() {
        let toml = r#"
[species]
name = "Determinism Test"

[trunk]
height = 5.0

[branches.level1]
count = 4
length = 2.0
"#;

        let species = Species::from_toml(toml).unwrap();

        let tree1 = generate_tree(&species, 12345);
        let tree2 = generate_tree(&species, 12345);

        assert_eq!(tree1.stem_count(), tree2.stem_count());

        for (s1, s2) in tree1.stems.iter().zip(tree2.stems.iter()) {
            assert_eq!(s1.id, s2.id);
            assert_eq!(s1.level, s2.level);
            assert_eq!(s1.segments.len(), s2.segments.len());
        }
    }
}
