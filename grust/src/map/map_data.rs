use hexx::Hex;

use crate::core::terrain::Biome;

// ---------------------------------------------------------------------------
// HexData — per-hex game state stored as SoA (Structure of Arrays).
//
// WHY SoA INSTEAD OF AoS (Array of Structs)?
//
//   AoS would be: Vec<HexRecord { hex, biome, elevation }>
//   SoA is:       Vec<Hex>, Vec<Biome>, Vec<i32>
//
//   During map generation we often iterate ALL biomes (to pick mesh variants)
//   without needing elevation, or ALL elevations for LOD decisions.
//   SoA lets the CPU cache only the array it actually needs — no wasted
//   cache lines loading fields we don't touch.
//
//   Index `i` is the stable identity of a hex tile:
//     hexes[i], biomes[i], elevations[i]  → always the same tile.
//
//   The index is what ECS components will store (a u32 / usize).
//   No HashMap lookup needed at runtime — just array indexing.
//
// ORDERING:
//   Hexes are sorted by (q, r) at build time and never reordered.
//   This guarantees deterministic index assignment across runs with the
//   same seed, which is required for save/load and multiplayer sync.
//
// FUTURE:
//   Adding a new per-hex attribute (e.g. moisture, region_id) = add one Vec.
//   SQLite persistence = one table, one column per Vec, one row per index.
// ---------------------------------------------------------------------------

pub struct MapData {
    /// Hex coordinates, sorted by (q, r). Index = stable tile identity.
    pub hexes: Vec<Hex>,
    /// Biome assigned to each hex. biomes[i] belongs to hexes[i].
    pub biomes: Vec<Biome>,
    /// Logical elevation step. elevations[i] belongs to hexes[i].
    pub elevations: Vec<i32>,
}

impl MapData {
    /// Build a MapData from parallel iterators.
    /// Sorts by (q, r) at construction to guarantee stable index order.
    pub fn from_unsorted(entries: impl IntoIterator<Item = (Hex, Biome, i32)>) -> Self {
        let mut entries: Vec<(Hex, Biome, i32)> = entries.into_iter().collect();
        entries.sort_by_key(|(hex, _, _)| (hex.x, hex.y));

        let len = entries.len();
        let mut hexes = Vec::with_capacity(len);
        let mut biomes = Vec::with_capacity(len);
        let mut elevations = Vec::with_capacity(len);

        for (hex, biome, elev) in entries {
            hexes.push(hex);
            biomes.push(biome);
            elevations.push(elev);
        }

        Self { hexes, biomes, elevations }
    }

    /// Total number of tiles.
    #[inline]
    pub fn len(&self) -> usize {
        self.hexes.len()
    }

    /// Returns the index for a hex coordinate, if it exists in the map.
    /// O(log n) binary search on the sorted hex vec.
    /// Use this for editor/tool queries, not hot-path iteration.
    pub fn index_of(&self, hex: Hex) -> Option<usize> {
        self.hexes
            .binary_search_by_key(&(hex.x, hex.y), |h| (h.x, h.y))
            .ok()
    }

    /// Access a single tile's data by index. Panics if out of bounds.
    #[inline]
    pub fn get(&self, idx: usize) -> (Hex, Biome, i32) {
        (self.hexes[idx], self.biomes[idx], self.elevations[idx])
    }
}