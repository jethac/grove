//! Grove - Procedural tree generation library
//!
//! A standalone procedural tree generation tool for real-time game engines.
//! Generates 3D tree meshes with LOD, wind animation data, and AI-generated textures.

pub mod constants;
pub mod math;
pub mod rng;
pub mod species;

pub use constants::*;
pub use rng::Rng;
pub use species::Species;

/// Placeholder for the tree generation module.
pub fn generate() {
    // TODO: Implement tree generation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        generate();
    }
}
