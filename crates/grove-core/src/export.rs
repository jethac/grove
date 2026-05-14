//! glTF 2.0 export for tree meshes.

use crate::{mesh::Submesh, LodMeshSet, Mesh};
use std::io::Write;
use std::path::Path;

/// Export configuration
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Export format
    pub format: ExportFormat,
    /// Enable Draco compression (requires separate processing)
    pub draco: bool,
    /// Include embedded placeholder textures
    pub embed_textures: bool,
    /// Add pivot_painter flag in extras
    pub pivot_painter_extras: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Glb,  // Binary glTF
    GlTf, // JSON + separate binary
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Glb,
            draco: false,
            embed_textures: false,
            pivot_painter_extras: true,
        }
    }
}

/// Export error types
#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Json(serde_json::Error),
    NoMeshes,
}

impl From<std::io::Error> for ExportError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for ExportError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Json(e) => write!(f, "JSON error: {}", e),
            Self::NoMeshes => write!(f, "No meshes to export"),
        }
    }
}

impl std::error::Error for ExportError {}

/// Export a single mesh to glTF
pub fn export_mesh(mesh: &Mesh, path: &Path, config: &ExportConfig) -> Result<(), ExportError> {
    if mesh.vertices.is_empty() {
        return Err(ExportError::NoMeshes);
    }

    let gltf_data = build_gltf_single(mesh, config)?;
    write_gltf(path, &gltf_data, config.format)?;
    Ok(())
}

/// Export LOD mesh set to glTF (multiple meshes in one file)
pub fn export_lod_meshes(
    lods: &LodMeshSet,
    path: &Path,
    config: &ExportConfig,
) -> Result<(), ExportError> {
    if lods.meshes.is_empty() {
        return Err(ExportError::NoMeshes);
    }

    let gltf_data = build_gltf_lods(lods, config)?;
    write_gltf(path, &gltf_data, config.format)?;
    Ok(())
}

/// Export LOD mesh set to GLB bytes (for WASM/in-memory use)
pub fn export_lod_meshes_to_bytes(lods: &LodMeshSet, config: &ExportConfig) -> Result<Vec<u8>, ExportError> {
    if lods.meshes.is_empty() {
        return Err(ExportError::NoMeshes);
    }

    let gltf_data = build_gltf_lods(lods, config)?;
    build_glb_bytes(&gltf_data)
}

/// Build GLB file format in memory
fn build_glb_bytes(data: &GltfData) -> Result<Vec<u8>, ExportError> {
    let json_bytes = serde_json::to_vec(&data.json)?;

    // Pad JSON to 4-byte alignment
    let json_padding = (4 - (json_bytes.len() % 4)) % 4;
    let json_chunk_length = json_bytes.len() + json_padding;

    // Pad binary to 4-byte alignment
    let bin_padding = (4 - (data.binary.len() % 4)) % 4;
    let bin_chunk_length = data.binary.len() + bin_padding;

    // Calculate total file size
    let total_length = 12  // GLB header
        + 8 + json_chunk_length  // JSON chunk header + data
        + 8 + bin_chunk_length; // BIN chunk header + data

    let mut buffer = Vec::with_capacity(total_length);

    // GLB header
    buffer.extend_from_slice(b"glTF"); // magic
    buffer.extend_from_slice(&2u32.to_le_bytes()); // version
    buffer.extend_from_slice(&(total_length as u32).to_le_bytes()); // length

    // JSON chunk
    buffer.extend_from_slice(&(json_chunk_length as u32).to_le_bytes()); // chunk length
    buffer.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // chunk type "JSON"
    buffer.extend_from_slice(&json_bytes);
    buffer.extend_from_slice(&vec![0x20u8; json_padding]); // padding with spaces

    // BIN chunk
    buffer.extend_from_slice(&(bin_chunk_length as u32).to_le_bytes()); // chunk length
    buffer.extend_from_slice(&0x004E4942u32.to_le_bytes()); // chunk type "BIN\0"
    buffer.extend_from_slice(&data.binary);
    buffer.extend_from_slice(&vec![0u8; bin_padding]); // padding with zeros

    Ok(buffer)
}

/// glTF data container (simplified representation)
struct GltfData {
    json: serde_json::Value,
    binary: Vec<u8>,
}

fn build_gltf_single(mesh: &Mesh, config: &ExportConfig) -> Result<GltfData, ExportError> {
    let mut binary = Vec::new();

    // Build vertex buffer
    let positions_offset = binary.len();
    let positions = build_positions_buffer(mesh);
    binary.extend_from_slice(&positions);

    let normals_offset = binary.len();
    let normals = build_normals_buffer(mesh);
    binary.extend_from_slice(&normals);

    let texcoord0_offset = binary.len();
    let texcoord0 = build_texcoord0_buffer(mesh);
    binary.extend_from_slice(&texcoord0);

    let texcoord1_offset = binary.len();
    let texcoord1 = build_texcoord1_buffer(mesh);
    binary.extend_from_slice(&texcoord1);

    let color0_offset = binary.len();
    let color0 = build_color0_buffer(mesh);
    binary.extend_from_slice(&color0);

    let indices_offset = binary.len();
    let indices = build_indices_buffer(mesh);
    binary.extend_from_slice(&indices);

    // Compute bounds
    let (min_pos, max_pos) = compute_bounds(mesh);

    // Build glTF JSON
    let json = build_gltf_json(
        &[MeshBufferInfo {
            name: "tree_lod0".to_string(),
            vertex_count: mesh.vertices.len(),
            index_count: mesh.indices.len(),
            positions_offset,
            normals_offset,
            texcoord0_offset,
            texcoord1_offset,
            color0_offset,
            indices_offset,
            min_pos,
            max_pos,
            submeshes: mesh.submeshes.clone(),
        }],
        binary.len(),
        config,
    )?;

    Ok(GltfData { json, binary })
}

fn build_gltf_lods(lods: &LodMeshSet, config: &ExportConfig) -> Result<GltfData, ExportError> {
    let mut binary = Vec::new();
    let mut mesh_infos = Vec::new();

    for lod in &lods.meshes {
        let mesh = &lod.mesh;
        if mesh.vertices.is_empty() {
            continue;
        }

        let positions_offset = binary.len();
        binary.extend_from_slice(&build_positions_buffer(mesh));

        let normals_offset = binary.len();
        binary.extend_from_slice(&build_normals_buffer(mesh));

        let texcoord0_offset = binary.len();
        binary.extend_from_slice(&build_texcoord0_buffer(mesh));

        let texcoord1_offset = binary.len();
        binary.extend_from_slice(&build_texcoord1_buffer(mesh));

        let color0_offset = binary.len();
        binary.extend_from_slice(&build_color0_buffer(mesh));

        let indices_offset = binary.len();
        binary.extend_from_slice(&build_indices_buffer(mesh));

        let (min_pos, max_pos) = compute_bounds(mesh);

        mesh_infos.push(MeshBufferInfo {
            name: format!("tree_lod{}", lod.index),
            vertex_count: mesh.vertices.len(),
            index_count: mesh.indices.len(),
            positions_offset,
            normals_offset,
            texcoord0_offset,
            texcoord1_offset,
            color0_offset,
            indices_offset,
            min_pos,
            max_pos,
            submeshes: mesh.submeshes.clone(),
        });
    }

    let json = build_gltf_json(&mesh_infos, binary.len(), config)?;
    Ok(GltfData { json, binary })
}

struct MeshBufferInfo {
    name: String,
    vertex_count: usize,
    index_count: usize,
    positions_offset: usize,
    normals_offset: usize,
    texcoord0_offset: usize,
    texcoord1_offset: usize,
    color0_offset: usize,
    indices_offset: usize,
    min_pos: [f32; 3],
    max_pos: [f32; 3],
    submeshes: Vec<Submesh>,
}

fn build_positions_buffer(mesh: &Mesh) -> Vec<u8> {
    let mut data = Vec::with_capacity(mesh.vertices.len() * 12);
    for v in &mesh.vertices {
        data.extend_from_slice(&v.position.x.to_le_bytes());
        data.extend_from_slice(&v.position.y.to_le_bytes());
        data.extend_from_slice(&v.position.z.to_le_bytes());
    }
    data
}

fn build_normals_buffer(mesh: &Mesh) -> Vec<u8> {
    let mut data = Vec::with_capacity(mesh.vertices.len() * 12);
    for v in &mesh.vertices {
        data.extend_from_slice(&v.normal.x.to_le_bytes());
        data.extend_from_slice(&v.normal.y.to_le_bytes());
        data.extend_from_slice(&v.normal.z.to_le_bytes());
    }
    data
}

fn build_texcoord0_buffer(mesh: &Mesh) -> Vec<u8> {
    let mut data = Vec::with_capacity(mesh.vertices.len() * 8);
    for v in &mesh.vertices {
        data.extend_from_slice(&v.uv.x.to_le_bytes());
        data.extend_from_slice(&v.uv.y.to_le_bytes());
    }
    data
}

fn build_texcoord1_buffer(mesh: &Mesh) -> Vec<u8> {
    let mut data = Vec::with_capacity(mesh.vertices.len() * 8);
    for v in &mesh.vertices {
        data.extend_from_slice(&v.uv2.x.to_le_bytes());
        data.extend_from_slice(&v.uv2.y.to_le_bytes());
    }
    data
}

fn build_color0_buffer(mesh: &Mesh) -> Vec<u8> {
    let mut data = Vec::with_capacity(mesh.vertices.len() * 16);
    for v in &mesh.vertices {
        data.extend_from_slice(&v.color.x.to_le_bytes());
        data.extend_from_slice(&v.color.y.to_le_bytes());
        data.extend_from_slice(&v.color.z.to_le_bytes());
        data.extend_from_slice(&v.color.w.to_le_bytes());
    }
    data
}

fn build_indices_buffer(mesh: &Mesh) -> Vec<u8> {
    let mut data = Vec::with_capacity(mesh.indices.len() * 4);
    for &idx in &mesh.indices {
        data.extend_from_slice(&idx.to_le_bytes());
    }
    data
}

fn compute_bounds(mesh: &Mesh) -> ([f32; 3], [f32; 3]) {
    if mesh.vertices.is_empty() {
        return ([0.0; 3], [0.0; 3]);
    }

    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];

    for v in &mesh.vertices {
        min[0] = min[0].min(v.position.x);
        min[1] = min[1].min(v.position.y);
        min[2] = min[2].min(v.position.z);
        max[0] = max[0].max(v.position.x);
        max[1] = max[1].max(v.position.y);
        max[2] = max[2].max(v.position.z);
    }

    (min, max)
}

fn build_gltf_json(
    meshes: &[MeshBufferInfo],
    buffer_size: usize,
    config: &ExportConfig,
) -> Result<serde_json::Value, serde_json::Error> {
    use serde_json::json;

    let mut accessors = Vec::new();
    let mut buffer_views = Vec::new();
    let mut gltf_meshes = Vec::new();

    let mut accessor_idx = 0;
    let mut buffer_view_idx = 0;

    for mesh_info in meshes {
        let vertex_count = mesh_info.vertex_count;
        let index_count = mesh_info.index_count;

        // Buffer views
        let pos_bv = buffer_view_idx;
        buffer_view_idx += 1;
        let norm_bv = buffer_view_idx;
        buffer_view_idx += 1;
        let tc0_bv = buffer_view_idx;
        buffer_view_idx += 1;
        let tc1_bv = buffer_view_idx;
        buffer_view_idx += 1;
        let col_bv = buffer_view_idx;
        buffer_view_idx += 1;
        let idx_bv = buffer_view_idx;
        buffer_view_idx += 1;

        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": mesh_info.positions_offset,
            "byteLength": vertex_count * 12,
            "target": 34962  // ARRAY_BUFFER
        }));
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": mesh_info.normals_offset,
            "byteLength": vertex_count * 12,
            "target": 34962
        }));
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": mesh_info.texcoord0_offset,
            "byteLength": vertex_count * 8,
            "target": 34962
        }));
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": mesh_info.texcoord1_offset,
            "byteLength": vertex_count * 8,
            "target": 34962
        }));
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": mesh_info.color0_offset,
            "byteLength": vertex_count * 16,
            "target": 34962
        }));
        buffer_views.push(json!({
            "buffer": 0,
            "byteOffset": mesh_info.indices_offset,
            "byteLength": index_count * 4,
            "target": 34963  // ELEMENT_ARRAY_BUFFER
        }));

        // Accessors
        let pos_acc = accessor_idx;
        accessor_idx += 1;
        let norm_acc = accessor_idx;
        accessor_idx += 1;
        let tc0_acc = accessor_idx;
        accessor_idx += 1;
        let tc1_acc = accessor_idx;
        accessor_idx += 1;
        let col_acc = accessor_idx;
        accessor_idx += 1;
        let idx_acc = accessor_idx;
        accessor_idx += 1;

        accessors.push(json!({
            "bufferView": pos_bv,
            "componentType": 5126,  // FLOAT
            "count": vertex_count,
            "type": "VEC3",
            "min": mesh_info.min_pos,
            "max": mesh_info.max_pos
        }));
        accessors.push(json!({
            "bufferView": norm_bv,
            "componentType": 5126,
            "count": vertex_count,
            "type": "VEC3"
        }));
        accessors.push(json!({
            "bufferView": tc0_bv,
            "componentType": 5126,
            "count": vertex_count,
            "type": "VEC2"
        }));
        accessors.push(json!({
            "bufferView": tc1_bv,
            "componentType": 5126,
            "count": vertex_count,
            "type": "VEC2"
        }));
        accessors.push(json!({
            "bufferView": col_bv,
            "componentType": 5126,
            "count": vertex_count,
            "type": "VEC4"
        }));
        accessors.push(json!({
            "bufferView": idx_bv,
            "componentType": 5125,  // UNSIGNED_INT
            "count": index_count,
            "type": "SCALAR"
        }));

        // Build primitives for submeshes (or single primitive if no submeshes)
        let primitives = if mesh_info.submeshes.is_empty() {
            vec![json!({
                "attributes": {
                    "POSITION": pos_acc,
                    "NORMAL": norm_acc,
                    "TEXCOORD_0": tc0_acc,
                    "TEXCOORD_1": tc1_acc,
                    "COLOR_0": col_acc
                },
                "indices": idx_acc,
                "mode": 4,  // TRIANGLES
                "material": 0
            })]
        } else {
            // For now, use single primitive with all indices
            // TODO: Split by submesh for proper material assignment
            vec![json!({
                "attributes": {
                    "POSITION": pos_acc,
                    "NORMAL": norm_acc,
                    "TEXCOORD_0": tc0_acc,
                    "TEXCOORD_1": tc1_acc,
                    "COLOR_0": col_acc
                },
                "indices": idx_acc,
                "mode": 4,
                "material": 0
            })]
        };

        gltf_meshes.push(json!({
            "name": mesh_info.name,
            "primitives": primitives
        }));
    }

    // Build materials
    let materials = vec![
        json!({
            "name": "bark",
            "pbrMetallicRoughness": {
                "baseColorFactor": [0.4, 0.3, 0.2, 1.0],
                "metallicFactor": 0.0,
                "roughnessFactor": 0.85
            },
            "doubleSided": false
        }),
        json!({
            "name": "leaves",
            "pbrMetallicRoughness": {
                "baseColorFactor": [0.2, 0.5, 0.2, 1.0],
                "metallicFactor": 0.0,
                "roughnessFactor": 0.6
            },
            "doubleSided": true,
            "alphaMode": "MASK",
            "alphaCutoff": 0.5
        }),
    ];

    // Build nodes (one per mesh)
    let nodes: Vec<_> = (0..meshes.len())
        .map(|i| json!({ "mesh": i, "name": &meshes[i].name }))
        .collect();

    let scene = json!({
        "name": "Tree",
        "nodes": (0..meshes.len()).collect::<Vec<_>>()
    });

    let mut root = json!({
        "asset": {
            "generator": "grove",
            "version": "2.0"
        },
        "scene": 0,
        "scenes": [scene],
        "nodes": nodes,
        "meshes": gltf_meshes,
        "materials": materials,
        "accessors": accessors,
        "bufferViews": buffer_views,
        "buffers": [{
            "byteLength": buffer_size
        }]
    });

    // Add pivot painter extras
    if config.pivot_painter_extras {
        root["extras"] = json!({
            "pivot_painter": true,
            "pivot_painter_version": "2.0"
        });
    }

    Ok(root)
}

fn write_gltf(path: &Path, data: &GltfData, format: ExportFormat) -> Result<(), std::io::Error> {
    match format {
        ExportFormat::Glb => write_glb(path, data),
        ExportFormat::GlTf => write_gltf_separate(path, data),
    }
}

fn write_glb(path: &Path, data: &GltfData) -> Result<(), std::io::Error> {
    let json_bytes = serde_json::to_vec(&data.json)?;

    // Pad JSON to 4-byte alignment
    let json_padding = (4 - (json_bytes.len() % 4)) % 4;
    let json_chunk_length = json_bytes.len() + json_padding;

    // Pad binary to 4-byte alignment
    let bin_padding = (4 - (data.binary.len() % 4)) % 4;
    let bin_chunk_length = data.binary.len() + bin_padding;

    // Calculate total file size
    let total_length = 12  // GLB header
        + 8 + json_chunk_length  // JSON chunk header + data
        + 8 + bin_chunk_length; // BIN chunk header + data

    let mut file = std::fs::File::create(path)?;

    // GLB header
    file.write_all(b"glTF")?; // magic
    file.write_all(&2u32.to_le_bytes())?; // version
    file.write_all(&(total_length as u32).to_le_bytes())?; // length

    // JSON chunk
    file.write_all(&(json_chunk_length as u32).to_le_bytes())?; // chunk length
    file.write_all(&0x4E4F534Au32.to_le_bytes())?; // chunk type "JSON"
    file.write_all(&json_bytes)?;
    file.write_all(&vec![0x20u8; json_padding])?; // padding with spaces

    // BIN chunk
    file.write_all(&(bin_chunk_length as u32).to_le_bytes())?; // chunk length
    file.write_all(&0x004E4942u32.to_le_bytes())?; // chunk type "BIN\0"
    file.write_all(&data.binary)?;
    file.write_all(&vec![0u8; bin_padding])?; // padding with zeros

    Ok(())
}

fn write_gltf_separate(path: &Path, data: &GltfData) -> Result<(), std::io::Error> {
    // Determine the binary filename
    let bin_filename = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| format!("{}.bin", s))
        .unwrap_or_else(|| "buffer.bin".to_string());

    // Update JSON to reference external buffer
    let mut json = data.json.clone();
    if let Some(buffers) = json.get_mut("buffers").and_then(|b| b.as_array_mut()) {
        if let Some(buffer) = buffers.first_mut() {
            buffer["uri"] = serde_json::Value::String(bin_filename.clone());
        }
    }

    // Write JSON file
    let json_path = path.with_extension("gltf");
    let json_file = std::fs::File::create(&json_path)?;
    serde_json::to_writer_pretty(json_file, &json)?;

    // Write binary file
    let bin_path = path.with_extension("bin");
    std::fs::write(&bin_path, &data.binary)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{mesh::MaterialType, Vertex};
    use glam::{Vec2, Vec3, Vec4};
    use std::fs;
    use tempfile::tempdir;

    fn create_test_mesh() -> Mesh {
        let mut mesh = Mesh::new();

        // Create a simple triangle
        mesh.vertices.push(Vertex::with_pivot_painter(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ));
        mesh.vertices.push(Vertex::with_pivot_painter(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::Y,
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 0.5),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
        ));
        mesh.vertices.push(Vertex::with_pivot_painter(
            Vec3::new(0.5, 1.0, 0.0),
            Vec3::Y,
            Vec2::new(0.5, 1.0),
            Vec2::new(1.0, 1.0),
            Vec4::new(0.5, 1.0, 0.0, 1.0),
        ));

        mesh.indices.extend_from_slice(&[0, 1, 2]);

        mesh.submeshes.push(Submesh {
            index_start: 0,
            index_count: 3,
            material: MaterialType::Bark,
        });

        mesh
    }

    #[test]
    fn test_export_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.format, ExportFormat::Glb);
        assert!(!config.draco);
        assert!(!config.embed_textures);
        assert!(config.pivot_painter_extras);
    }

    #[test]
    fn test_export_empty_mesh_error() {
        let mesh = Mesh::new();
        let config = ExportConfig::default();
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.glb");

        let result = export_mesh(&mesh, &path, &config);
        assert!(matches!(result, Err(ExportError::NoMeshes)));
    }

    #[test]
    fn test_compute_bounds() {
        let mesh = create_test_mesh();
        let (min, max) = compute_bounds(&mesh);

        assert_eq!(min[0], 0.0);
        assert_eq!(min[1], 0.0);
        assert_eq!(min[2], 0.0);
        assert_eq!(max[0], 1.0);
        assert_eq!(max[1], 1.0);
        assert_eq!(max[2], 0.0);
    }

    #[test]
    fn test_compute_bounds_empty() {
        let mesh = Mesh::new();
        let (min, max) = compute_bounds(&mesh);

        assert_eq!(min, [0.0; 3]);
        assert_eq!(max, [0.0; 3]);
    }

    #[test]
    fn test_build_positions_buffer() {
        let mesh = create_test_mesh();
        let buffer = build_positions_buffer(&mesh);

        // 3 vertices * 3 floats * 4 bytes = 36 bytes
        assert_eq!(buffer.len(), 36);
    }

    #[test]
    fn test_build_normals_buffer() {
        let mesh = create_test_mesh();
        let buffer = build_normals_buffer(&mesh);

        assert_eq!(buffer.len(), 36);
    }

    #[test]
    fn test_build_texcoord_buffers() {
        let mesh = create_test_mesh();

        let tc0 = build_texcoord0_buffer(&mesh);
        let tc1 = build_texcoord1_buffer(&mesh);

        // 3 vertices * 2 floats * 4 bytes = 24 bytes
        assert_eq!(tc0.len(), 24);
        assert_eq!(tc1.len(), 24);
    }

    #[test]
    fn test_build_color_buffer() {
        let mesh = create_test_mesh();
        let buffer = build_color0_buffer(&mesh);

        // 3 vertices * 4 floats * 4 bytes = 48 bytes
        assert_eq!(buffer.len(), 48);
    }

    #[test]
    fn test_build_indices_buffer() {
        let mesh = create_test_mesh();
        let buffer = build_indices_buffer(&mesh);

        // 3 indices * 4 bytes = 12 bytes
        assert_eq!(buffer.len(), 12);
    }

    #[test]
    fn test_export_mesh_glb() {
        let mesh = create_test_mesh();
        let config = ExportConfig::default();
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.glb");

        let result = export_mesh(&mesh, &path, &config);
        assert!(result.is_ok());

        // Verify file exists
        assert!(path.exists());

        // Verify GLB magic bytes
        let data = fs::read(&path).unwrap();
        assert!(data.len() >= 12);
        assert_eq!(&data[0..4], b"glTF");
        assert_eq!(u32::from_le_bytes([data[4], data[5], data[6], data[7]]), 2); // version
    }

    #[test]
    fn test_export_mesh_gltf() {
        let mesh = create_test_mesh();
        let config = ExportConfig {
            format: ExportFormat::GlTf,
            ..Default::default()
        };
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.gltf");

        let result = export_mesh(&mesh, &path, &config);
        assert!(result.is_ok());

        // Verify both files exist
        assert!(path.exists());
        let bin_path = path.with_extension("bin");
        assert!(bin_path.exists());

        // Verify JSON is valid
        let json_data = fs::read_to_string(&path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&json_data).unwrap();

        assert_eq!(json["asset"]["version"], "2.0");
        assert_eq!(json["asset"]["generator"], "grove");
    }

    #[test]
    fn test_export_lod_meshes_empty() {
        let lods = LodMeshSet { meshes: vec![] };
        let config = ExportConfig::default();
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty_lods.glb");

        let result = export_lod_meshes(&lods, &path, &config);
        assert!(matches!(result, Err(ExportError::NoMeshes)));
    }

    #[test]
    fn test_gltf_json_structure() {
        let mesh = create_test_mesh();
        let config = ExportConfig::default();
        let gltf_data = build_gltf_single(&mesh, &config).unwrap();

        let json = &gltf_data.json;

        // Verify required glTF fields
        assert!(json.get("asset").is_some());
        assert!(json.get("scene").is_some());
        assert!(json.get("scenes").is_some());
        assert!(json.get("nodes").is_some());
        assert!(json.get("meshes").is_some());
        assert!(json.get("accessors").is_some());
        assert!(json.get("bufferViews").is_some());
        assert!(json.get("buffers").is_some());
        assert!(json.get("materials").is_some());

        // Verify extras for pivot painter
        assert!(json.get("extras").is_some());
        assert_eq!(json["extras"]["pivot_painter"], true);
    }

    #[test]
    fn test_gltf_materials() {
        let mesh = create_test_mesh();
        let config = ExportConfig::default();
        let gltf_data = build_gltf_single(&mesh, &config).unwrap();

        let materials = gltf_data.json["materials"].as_array().unwrap();
        assert_eq!(materials.len(), 2);

        // Bark material
        assert_eq!(materials[0]["name"], "bark");
        assert_eq!(materials[0]["doubleSided"], false);

        // Leaves material
        assert_eq!(materials[1]["name"], "leaves");
        assert_eq!(materials[1]["doubleSided"], true);
        assert_eq!(materials[1]["alphaMode"], "MASK");
    }

    #[test]
    fn test_export_without_pivot_painter_extras() {
        let mesh = create_test_mesh();
        let config = ExportConfig {
            pivot_painter_extras: false,
            ..Default::default()
        };
        let gltf_data = build_gltf_single(&mesh, &config).unwrap();

        assert!(gltf_data.json.get("extras").is_none());
    }

    #[test]
    fn test_export_error_display() {
        let io_err = ExportError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert!(io_err.to_string().contains("IO error"));

        let no_mesh_err = ExportError::NoMeshes;
        assert!(no_mesh_err.to_string().contains("No meshes"));
    }
}
