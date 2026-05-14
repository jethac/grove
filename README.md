# Grove

A procedural tree generator for real-time game engines. Generates 3D tree meshes with LOD support, wind animation data, and exports to glTF 2.0.

## Features

- **Weber-Penn branching algorithm** - Biologically-inspired recursive branching with configurable parameters
- **Multiple LOD levels** - Automatic generation of level-of-detail meshes with configurable triangle budgets
- **Leaf systems** - Polygon, cross-billboard, and billboard leaf geometries
- **Pivot Painter 2.0** - Wind animation vertex data compatible with Unreal Engine 5
- **glTF 2.0 export** - Binary (.glb) or JSON (.gltf) output with PBR materials
- **Species presets** - Oak, Pine, Palm, and Willow included

## Installation

### From source

```bash
git clone https://github.com/jethac/grove.git
cd grove
cargo build --release
```

The binary will be at `target/release/grove`.

## Usage

### Generate a tree

```bash
grove generate -s presets/species/oak.toml -o tree.glb
```

### Generate multiple variants

```bash
grove generate -s presets/species/pine.toml -n 10 --seed 42 -o forest/pine.glb
```

Output files will be named `pine_0.glb`, `pine_1.glb`, etc.

### Show species information

```bash
grove info -s presets/species/willow.toml
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-s, --species <FILE>` | Species TOML file | required |
| `-o, --output <PATH>` | Output path | `tree.glb` |
| `-n, --count <N>` | Number of variants | `1` |
| `--seed <N>` | Random seed | random |
| `--lod <all\|0\|1\|2\|3>` | LOD level(s) to export | `all` |
| `--format <glb\|gltf>` | Output format | `glb` |
| `--lod-preset <PRESET>` | LOD quality preset | `balanced` |
| `-v, --verbose` | Verbose output | off |

### LOD Presets

| Preset | Levels | Max Triangles |
|--------|--------|---------------|
| `ultra` | 5 | 50,000 |
| `high_quality` | 4 | 30,000 |
| `balanced` | 3 | 8,000 |
| `mobile` | 3 | 3,000 |
| `minimal` | 2 | 1,500 |

## Species Files

Trees are defined in TOML files. Example:

```toml
[species]
name = "Oak"
scientific = "Quercus robur"

[trunk]
height = 6.0
radius = 0.45
taper = 0.75
curve = 8.0
segments = 8

[branches.level1]
count = 6
length = 4.5
angle = 55.0
rotation = 137.5
gravity = -0.15

[crown]
shape = "spherical"
offset = 0.35

[leaves]
count = 3000
size = 0.12
geometry = "cross_billboard"
```

See `presets/species/` for complete examples.

### Crown Shapes

- `spherical` - Round, spreading crown (oak, maple)
- `conical` - Triangular profile (pine, spruce)
- `hemispherical` - Dome-shaped (palm)
- `flame` - Pointed oval (cypress)
- `columnar` - Tall and narrow (poplar)

### Leaf Geometries

- `polygon` - Actual leaf-shaped mesh, no alpha testing needed
- `cross_billboard` - Two quads at 90°, good balance of quality/performance
- `billboard` - Single quad, cheapest option
- `none` - No leaves (used for lowest LOD)

## Output Format

Grove exports glTF 2.0 with:

- Multiple meshes (one per LOD level)
- Vertex attributes: `POSITION`, `NORMAL`, `TEXCOORD_0`, `TEXCOORD_1`, `COLOR_0`
- PBR materials for bark and leaves
- Pivot Painter data encoded in `TEXCOORD_1` and `COLOR_0`

### Pivot Painter Data

For wind animation in game engines:

| Attribute | Channel | Data |
|-----------|---------|------|
| `TEXCOORD_1.x` | U | Branch depth (0=trunk, 1=leaf) |
| `TEXCOORD_1.y` | V | Phase offset |
| `COLOR_0.xyz` | RGB | Pivot position (normalized) |
| `COLOR_0.w` | A | Stiffness (0=flexible, 1=rigid) |

## Project Structure

```
grove/
├── crates/
│   ├── grove-core/     # Core generation library
│   ├── grove-cli/      # Command-line interface
│   ├── grove-wasm/     # WebAssembly bindings (WIP)
│   └── grove-ffi/      # C FFI for engine plugins (WIP)
└── presets/
    └── species/        # Species TOML files
```

## Library Usage

```rust
use grove_core::{Species, generate_tree, generate_lod_meshes, export_lod_meshes, ExportConfig};
use std::path::Path;

let species = Species::from_file(Path::new("oak.toml")).unwrap();
let tree = generate_tree(&species, 12345);
let lods = generate_lod_meshes(&tree, &species);

export_lod_meshes(&lods, Path::new("tree.glb"), &ExportConfig::default()).unwrap();
```

## License

MIT
