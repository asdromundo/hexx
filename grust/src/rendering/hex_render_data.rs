use crate::map::mesh_registry::MeshRegistry;

// ---------------------------------------------------------------------------
// HexRenderData — internal Rust representation.
//
// Produced by map generation after all wedge mesh IDs have been resolved.
// This type lives entirely in Rust; it is NEVER sent to Godot directly.
//
// `wedge_keys`: 6 namespaced asset keys, one per wedge (direction 0–5).
//   Each key was resolved via MeshRegistry::key_for_id() after the spatial-
//   hash variant selection in get_mesh_id().
//
// `elevation`: logical elevation step (not world-space height).
// `center_feature`: optional feature mesh key index; None = no feature.
// ---------------------------------------------------------------------------
pub struct HexRenderData {
    /// Resolved asset keys for each of the 6 wedge triangles.
    /// Index matches hexx direction convention (0 = East, clockwise).
    pub wedge_keys: [Box<str>; 6],
    pub elevation: i32,
    pub center_feature: Option<Box<str>>,
    /// World-space XZ position of the hex centre (Y handled via elevation).
    /// Computed from HexLayout at generation time so Godot never needs to.
    pub world_pos: hexx::Vec2,
}

impl HexRenderData {
    pub fn from_ids(
        wedge_ids: [i32; 6],
        elevation: i32,
        center_feature_id: Option<i32>,
        world_pos: hexx::Vec2,
        registry: &MeshRegistry,
    ) -> Self {
        let wedge_keys = wedge_ids.map(|id| registry.key_for_id(id).into());
        let center_feature = center_feature_id.map(|id| registry.key_for_id(id).into());

        Self { wedge_keys, elevation, center_feature, world_pos }
    }
}

// ---------------------------------------------------------------------------
// HexRenderDataFfi — the type that actually crosses into Godot.
//
// Constraints:
//   - GDExtension bindings expect Godot types (GString, Array, etc.).
//   - No raw pointers or Rust-only types at the boundary.
//   - Godot's PackedStringArray is the most efficient way to pass
//     a fixed-size string array across the FFI without heap allocation
//     per element on the Rust side.
//
// This type is deliberately thin — it carries only what Godot needs to
// call MeshAssetRegistry.instantiate(key) for each wedge.
// ---------------------------------------------------------------------------
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub struct HexRenderDataFfi {
    base: Base<RefCounted>,

    /// Keys for the 6 wedges, in direction order.
    /// Godot reads this and calls MeshAssetRegistry.instantiate(key).
    #[var]
    pub wedge_keys: PackedStringArray,

    /// Logical elevation. Godot converts this to world-space Y offset.
    #[var]
    pub elevation: i32,

    /// World-space XZ position of the hex centre.
    /// Use as Vector3(world_pos.x, elevation * ELEVATION_STEP, world_pos.y) in the renderer.
    #[var]
    pub world_pos: Vector2,

    /// Key for the center feature mesh, or empty string if none.
    #[var]
    pub center_feature_key: GString,
}

#[godot_api]
impl HexRenderDataFfi {
    /// Construct from the internal HexRenderData.
    /// Called once per hex after map generation — not in the hot path.
    pub fn from_render_data(data: &HexRenderData) -> Gd<Self> {
        let mut wedge_keys = PackedStringArray::new();
        for key in &data.wedge_keys {
            wedge_keys.push(&GString::from(key.as_ref()));
        }

        let center_feature_key = data
            .center_feature
            .as_deref()
            .map(GString::from)
            .unwrap_or_default();

        Gd::from_init_fn(|base| Self {
            base,
            wedge_keys,
            elevation: data.elevation,
            world_pos: Vector2::new(data.world_pos.x, data.world_pos.y),
            center_feature_key,
        })
    }

    /// Convenience: get the key for a single wedge by direction index (0–5).
    /// Avoids allocating the full array on the GDScript side for spot queries.
    #[func]
    pub fn wedge_key(&self, direction: i32) -> GString {
        if direction < 0 || direction >= 6 {
            godot::global::push_error(
                &[format!("HexRenderDataFfi::wedge_key: direction {} out of range 0–5", direction).to_variant()]
            );
            return GString::default();
        }
        self.wedge_keys
            .get(direction as usize)
            .unwrap_or_default()
    }

    /// True if this hex has a center feature mesh.
    #[func]
    pub fn has_center_feature(&self) -> bool {
        !self.center_feature_key.is_empty()
    }
}