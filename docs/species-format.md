# Species TOML format

A species document is the authoritative input to every Grove surface (CLI,
WASM workbench, C FFI). This file documents the schema; `presets/species/`
contains complete examples.

```toml
[species]
name = "Oak"                       # required
scientific = "Quercus robur"       # optional

[trunk]
height = 6.0                       # metres, default 8.0
height_variance = 0.15             # 0..1 randomisation
radius = 0.45                      # metres, default 0.45
taper = 0.75                       # 0..1, default 0.7
curve = 8.0                        # degrees
curve_variance = 4.0               # degrees
curve_back = 3.0                   # degrees, corrective counter-curve
segments = 8                       # ring resolution, default 8

# Up to three recursive branch levels. Omit a section to disable the level.
[branches.level1]
count = 6                          # children per parent
count_variance = 2                 # ± randomisation
length = 4.5                       # metres
length_variance = 0.25             # 0..1
radius_ratio = 0.55                # relative to parent radius
angle = 55.0                       # degrees from parent axis
angle_variance = 20.0
rotation = 137.5                   # degrees around parent (golden angle default)
gravity = -0.15                    # -1..1, negative pulls upward
curve = 25.0                       # degrees along the branch
curve_variance = 10.0
segments = 6

[branches.level2]  # ...           # same fields, typically thinner/shorter
[branches.level3]  # ...

[crown]
shape = "spherical"                # spherical | conical | hemispherical |
                                   # flame | columnar
offset = 0.35                      # 0..1 up the trunk where the crown starts
density = 1.0                      # multiplier on leaf count
width_ratio = 1.2                  # width relative to height

[leaves]
count = 3000                       # target count at LOD0
min_level = 2                      # branches below this level get no leaves
size = 0.12                        # metres
size_variance = 0.25               # 0..1
distribution = "both"              # endpoint | along_branch | both
geometry = "cross_billboard"       # polygon | cross_billboard | billboard | none
up_influence = 0.25                # 0..1, how strongly leaves tilt upward

[textures]                         # authoring hints consumed by art pipeline
bark_prompt = "..."
leaf_prompt = "..."

[lod]
preset = "balanced"                # ultra | high_quality | balanced | mobile |
                                   # minimal | custom
count = 3                          # optional override of preset level count

# With preset = "custom", declare levels explicitly:
# [[lod.levels]]
# index = 0
# name = "high"
# target_triangles = 8000
# branch_levels = 3
# leaf_geometry = "cross_billboard"
# leaf_reduction = 0.0
# ring_resolution = [16, 10, 6, 4] # per-level ring segments
# screen_height = 1.0              # switch threshold (fraction of screen)
# crown_impostor = false

[platform]
target = "modern_pc"               # modern_pc | mobile | switch | quest |
                                   # web | universal
```

## Notes

- All floats are f32; counts are u32. `*_variance` values are ± ranges applied
  per seed, so the same document + seed is fully deterministic.
- `leaf_reduction` in custom LOD levels scales `leaves.count`
  (1.0 = remove none, 0.0 = remove all).
- Unknown sections or keys are rejected — the document must stay strictly
  inside this schema (that is what makes a bad edit surface in the workbench
  source panel instead of silently no-oping).
