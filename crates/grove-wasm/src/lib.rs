//! # grove-wasm
//!
//! WebAssembly bindings for the grove procedural tree generator.
//!
//! This crate provides a WebAssembly interface for using grove in web browsers
//! and other WASM-compatible environments. It wraps the core functionality
//! from `grove-core` with wasm-bindgen bindings.
//!
//! ## Features
//!
//! - Generate trees directly in the browser
//! - Export to glTF for use with Three.js, Babylon.js, etc.
//! - Real-time parameter adjustment with immediate preview
//!
//! ## Example (JavaScript)
//!
//! ```javascript
//! import init, { generate_tree } from 'grove-wasm';
//!
//! await init();
//! const gltfData = generate_tree({ species: 'oak', seed: 42 });
//! ```

use wasm_bindgen::prelude::*;

/// Initialize the WASM module.
#[wasm_bindgen(start)]
pub fn init() {
    // TODO: Set up panic hook for better error messages
}

/// Generate a tree and return it as glTF binary data.
#[wasm_bindgen]
pub fn generate_tree() -> Vec<u8> {
    // TODO: Implement using grove_core::generate_tree()
    // TODO: Return actual glTF data
    Vec::new()
}
