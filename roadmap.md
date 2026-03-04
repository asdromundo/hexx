project/
├── scenes/
│   └── grid_3d_map.tscn
├── scripts/
│   ├── Grid3DMap.gd
│   ├── HexRenderer.gd
│   └── autoloads/
│       └── MeshAssetRegistry.gd
├── assets/
│   └── meshes/terrain/
└── grust/                          ← invisible al editor
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── core/
        │   └── terrain.rs          ← Biome enum
        ├── map/
        │   ├── map_data.rs
        │   ├── map_generator.rs
        │   └── mesh_registry.rs
        ├── rendering/
        │   └── hex_render_data.rs
        └── gdscript/               ← todo lo que expone GDExtension
            ├── hex_map.rs          ← HexMap (RefCounted)
            ├── hex_utils.rs        ← HexGridUtils
            ├── hex3d_grid.rs       ← Hex3DGrid (Node3D)
            └── map_generator.rs    ← MapGeneratorNode + MapDataRef