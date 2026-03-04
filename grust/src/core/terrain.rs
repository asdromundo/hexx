// core/terrain.rs

/// The logical terrain type of a hex tile.
///
/// This is the source-of-truth biome for a hex as a grid entity.
/// It is NOT a mesh type — mesh selection is derived from the Cartesian
/// product of two adjacent Biomes via MeshRegistry::get_mesh_id().
///
/// Discriminants are stable and used as indices into GDScript BIOME_NAMES.
/// Never reorder or insert variants in the middle.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Biome {
    Grass = 0,
    Water = 1,
}

impl Biome {
    /// Total number of biome variants.
    /// Used to size the MeshRegistry LUT: n * n entries covers all transitions.
    pub const COUNT: usize = 2;

    /// All variants in discriminant order.
    /// Use for iterating the full biome set (e.g. registry validation).
    pub const ALL: [Biome; Self::COUNT] = [Biome::Grass, Biome::Water];
}