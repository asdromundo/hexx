use godot::prelude::*;

use crate::map::map_data::MapData;
// ---------------------------------------------------------------------------
// MapDataRef — GDExtension-exposed RefCounted wrapper.
//
// WHY A WRAPPER?
//   `MapData` is a plain Rust struct. GDExtension requires GodotClass.
//   We wrap it in a RefCounted so GDScript can hold a reference to it
//   without us needing to clone the data.
//
//   GDScript can query individual tiles (for UI tooltips, editor tools)
//   but cannot mutate the data — all setters are intentionally absent.
// ---------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct MapDataRef {
    base: Base<RefCounted>,
    pub(crate) data: MapData,
}

#[godot_api]
impl MapDataRef {
    /// Total number of hex tiles in the map.
    #[func]
    pub fn tile_count(&self) -> i32 {
        self.data.len() as i32
    }

    /// Returns the biome integer for tile at index.
    /// Biome variants map 1:1 to your Biome enum discriminants.
    #[func]
    pub fn biome_at(&self, idx: i32) -> i32 {
        if idx < 0 || idx as usize >= self.data.len() {
            godot_error!("MapDataRef::biome_at: index {} out of range", idx);
            return -1;
        }
        self.data.biomes[idx as usize] as i32
    }

    /// Returns the logical elevation for tile at index.
    #[func]
    pub fn elevation_at(&self, idx: i32) -> i32 {
        if idx < 0 || idx as usize >= self.data.len() {
            godot_error!("MapDataRef::elevation_at: index {} out of range", idx);
            return 0;
        }
        self.data.elevations[idx as usize]
    }

    /// Returns the hex coordinate as Vector2i (q, r) for tile at index.
    #[func]
    pub fn hex_coord_at(&self, idx: i32) -> Vector2i {
        if idx < 0 || idx as usize >= self.data.len() {
            godot_error!("MapDataRef::hex_coord_at: index {} out of range", idx);
            return Vector2i::ZERO;
        }
        let hex = self.data.hexes[idx as usize];
        Vector2i::new(hex.x, hex.y)
    }

    /// Construct from MapData — called only from Rust (map generator).
    pub fn from_map_data(data: MapData) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { base, data })
    }
}