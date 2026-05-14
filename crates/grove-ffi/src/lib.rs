//! # grove-ffi
//!
//! C FFI bindings for the grove procedural tree generator.
//!
//! This crate provides a C-compatible interface for integrating grove into
//! game engines and other native applications. It can be compiled as both
//! a dynamic library (cdylib) and a static library (staticlib).
//!
//! ## Supported Engines
//!
//! - Unreal Engine (via plugin)
//! - Unity (via native plugin)
//! - Godot (via GDExtension)
//! - Custom engines via direct C API
//!
//! ## Safety
//!
//! All FFI functions are marked as `unsafe` and require proper handling
//! of raw pointers. See the C header file for usage documentation.

use std::ffi::c_void;

/// Opaque handle to a generated tree.
pub type GroveTreeHandle = *mut c_void;

/// Generate a tree with default parameters.
///
/// # Safety
///
/// The returned handle must be freed with `grove_tree_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn grove_tree_generate() -> GroveTreeHandle {
    grove_core::generate();
    // TODO: Return actual tree handle
    std::ptr::null_mut()
}

/// Free a tree handle.
///
/// # Safety
///
/// The handle must have been returned by `grove_tree_generate` and
/// must not be used after this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn grove_tree_free(_handle: GroveTreeHandle) {
    // TODO: Implement proper cleanup
}

/// Get the version string of the grove library.
#[unsafe(no_mangle)]
pub extern "C" fn grove_version() -> *const std::ffi::c_char {
    // Include null terminator
    b"0.1.0\0".as_ptr() as *const std::ffi::c_char
}
