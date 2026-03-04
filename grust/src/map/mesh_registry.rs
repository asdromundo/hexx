use std::collections::HashMap;
use crate::core::terrain::Biome;

// ---------------------------------------------------------------------------
// Tags — gameplay-relevant metadata Rust cares about.
// Godot never needs to read these; they're for rules/logic (e.g. pathfinding
// cost modifiers, effect triggers, future bevy_ecs queries).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeshTag {
    /// Pure flat tile, no transition geometry.
    Flat,
    /// Blends two biomes (edge/transition piece).
    Transition,
    /// Geometry sits at a hex corner (vertex-touching neighbour blend).
    Corner,
    /// Carries a decoration (rock, tree stump, etc.) baked into the mesh.
    WithFeature,
    /// Custom tag for data-pack authors.
    Custom(String),
}

// ---------------------------------------------------------------------------
// MeshEntry — one logical mesh known to the Rust side.
//
// `key` is the shared contract string both Rust and Godot agree on.
// Format: "<pack_namespace>:<asset_name>"  e.g. "base:grass_flat_a"
//
// `id` is a dense integer assigned at registry build time.
// It is an *internal* optimisation (used in HexRenderData::wedges[]).
// It is NOT a stable public identifier across sessions or mod packs.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MeshEntry {
    /// Dense integer for runtime lookup (index into flat arrays, etc.).
    pub id: i32,
    /// Namespaced string key shared with the Godot asset registry.
    pub key: Box<str>,
    /// Gameplay/logic tags — Godot does not need to parse these.
    pub tags: Vec<MeshTag>,
}

// ---------------------------------------------------------------------------
// MeshRegistry — Rust-side owner of all MeshEntry definitions.
//
// Responsibilities:
//   • Map (Biome, Biome) pairs → candidate entry IDs (variant pool).
//   • Resolve an ID back to its MeshEntry (for emitting keys to Godot).
//   • Assign stable-within-session integer IDs at build time.
//
// NOT responsible for:
//   • Asset paths  (Godot owns those)
//   • PackedScene handles (Godot owns those)
//   • Loading / instantiation (Godot owns that)
// ---------------------------------------------------------------------------

pub struct MeshRegistry {
    /// All known entries, indexed by their `id` field for O(1) lookup.
    entries: Vec<MeshEntry>,
    /// Biome-pair → list of entry IDs (variant pool for spatial hashing).
    biome_lut: HashMap<(Biome, Biome), Vec<i32>>,
    /// Key → id for reverse lookups and de-duplication at build time.
    key_index: HashMap<Box<str>, i32>,
    /// Returned when no biome-pair match is found.
    fallback_id: i32,
}

impl MeshRegistry {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    pub fn builder() -> MeshRegistryBuilder {
        MeshRegistryBuilder::new()
    }

    /// Convenience constructor for tests / prototyping.
    pub fn new_mock() -> Self {
        MeshRegistryBuilder::new()
            // Grass-Grass variants
            .entry("base:grass_flat_a",    &[MeshTag::Flat])
            .entry("base:grass_flat_b",    &[MeshTag::Flat])
            // Water-Water variants
            .entry("base:water_flat_a",    &[MeshTag::Flat])
            .entry("base:water_flat_b",    &[MeshTag::Flat])
            // Grass→Water transition
            .entry("base:grass_water_a",   &[MeshTag::Transition])
            .entry("base:grass_water_b",   &[MeshTag::Transition])
            // Water→Grass transition
            .entry("base:water_grass_a",   &[MeshTag::Transition])
            .entry("base:water_grass_b",   &[MeshTag::Transition])
            // Fallback (should never appear in a well-configured registry)
            .entry("base:missing",         &[MeshTag::Flat])
            // .biome_pair(Biome::Grass, Biome::Grass, &["base:grass_flat_a",  "base:grass_flat_b"])
            // .biome_pair(Biome::Water, Biome::Water, &["base:water_flat_a",  "base:water_flat_b"])
            // .biome_pair(Biome::Grass, Biome::Water, &["base:grass_water_a", "base:grass_water_b"])
            // .biome_pair(Biome::Water, Biome::Grass, &["base:water_grass_a", "base:water_grass_b"])
            .biome_pair(Biome::Grass, Biome::Grass, &["base:grass_flat_a"])
            .biome_pair(Biome::Water, Biome::Water, &["base:water_flat_a"])
            .biome_pair(Biome::Grass, Biome::Water, &["base:grass_water_a"])
            .biome_pair(Biome::Water, Biome::Grass, &["base:water_grass_a"])
            .fallback("base:missing")
            .build()
    }

    // ------------------------------------------------------------------
    // Queries
    // ------------------------------------------------------------------

    /// Returns the integer ID for the best-matching mesh given the two
    /// biomes on either side of this wedge.  Spatial hash selects among
    /// variant candidates so neighbouring tiles don't look identical.
    #[inline]
    pub fn get_mesh_id(&self, inner: Biome, outer: Biome, q: i32, r: i32, d: usize) -> i32 {
        if let Some(variants) = self.biome_lut.get(&(inner, outer)) {
            let idx = spatial_hash(q, r, d) as usize % variants.len();
            return variants[idx];
        }
        self.fallback_id
    }

    /// Resolves an integer ID back to its full entry.
    /// Returns the fallback entry if the id is out of range.
    #[inline]
    pub fn entry_by_id(&self, id: i32) -> &MeshEntry {
        let idx = id.max(0) as usize;
        self.entries
            .get(idx)
            .unwrap_or_else(|| &self.entries[self.fallback_id.max(0) as usize])
    }

    /// Returns the namespaced key for a given id.
    /// This is what gets emitted to Godot's asset registry.
    #[inline]
    pub fn key_for_id(&self, id: i32) -> &str {
        &self.entry_by_id(id).key
    }

    /// Iterates all registered entries — useful for Godot to pre-warm its
    /// asset registry on startup (load every key it will ever see).
    pub fn all_entries(&self) -> impl Iterator<Item = &MeshEntry> {
        self.entries.iter()
    }
}

// ---------------------------------------------------------------------------
// MeshRegistryBuilder
// ---------------------------------------------------------------------------

pub struct MeshRegistryBuilder {
    entries: Vec<MeshEntry>,
    key_index: HashMap<Box<str>, i32>,
    biome_lut: HashMap<(Biome, Biome), Vec<i32>>,
    fallback_key: Option<Box<str>>,
}

impl MeshRegistryBuilder {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            key_index: HashMap::new(),
            biome_lut: HashMap::new(),
            fallback_key: None,
        }
    }

    /// Register a mesh entry by its namespaced key.
    /// Duplicate keys are silently ignored (first registration wins).
    pub fn entry(mut self, key: &str, tags: &[MeshTag]) -> Self {
        let boxed: Box<str> = key.into();
        if !self.key_index.contains_key(&boxed) {
            let id = self.entries.len() as i32;
            self.key_index.insert(boxed.clone(), id);
            self.entries.push(MeshEntry {
                id,
                key: boxed,
                tags: tags.to_vec(),
            });
        }
        self
    }

    /// Bind a (Biome, Biome) pair to a set of variant keys.
    /// All keys must already be registered via `.entry()`.
    /// Panics in debug builds if a key is missing (misconfiguration guard).
    pub fn biome_pair(mut self, inner: Biome, outer: Biome, keys: &[&str]) -> Self {
        let ids: Vec<i32> = keys
            .iter()
            .map(|k| {
                *self.key_index.get(*k).unwrap_or_else(|| {
                    panic!(
                        "MeshRegistryBuilder: key '{}' used in biome_pair before being registered",
                        k
                    )
                })
            })
            .collect();

        self.biome_lut.insert((inner, outer), ids);
        self
    }

    /// Set the fallback mesh key (shown when no biome pair matches).
    pub fn fallback(mut self, key: &str) -> Self {
        self.fallback_key = Some(key.into());
        self
    }

    pub fn build(self) -> MeshRegistry {
        let fallback_id = self
            .fallback_key
            .as_deref()
            .and_then(|k| self.key_index.get(k))
            .copied()
            .unwrap_or(i32::MIN);

        MeshRegistry {
            entries: self.entries,
            biome_lut: self.biome_lut,
            key_index: self.key_index,
            fallback_id,
        }
    }
}

// ---------------------------------------------------------------------------
// Spatial hash (unchanged from original)
// ---------------------------------------------------------------------------

#[inline(always)]
fn spatial_hash(q: i32, r: i32, d: usize) -> u32 {
    let mut h = (q as u32).wrapping_mul(0x9E3779B1);
    h ^= (r as u32).wrapping_mul(0x85EBCA6B);
    h ^= (d as u32).wrapping_mul(0xC2B2AE35);
    h ^= h >> 16;
    h
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_registry_resolves_all_biome_pairs() {
        let reg = MeshRegistry::new_mock();

        let pairs = [
            (Biome::Grass, Biome::Grass),
            (Biome::Water, Biome::Water),
            (Biome::Grass, Biome::Water),
            (Biome::Water, Biome::Grass),
        ];

        for (inner, outer) in pairs {
            let id = reg.get_mesh_id(inner, outer, 3, 7, 2);
            assert!(id >= 0, "fallback triggered for {:?}/{:?}", inner, outer);
            let key = reg.key_for_id(id);
            assert!(key.contains(':'), "key '{}' missing namespace separator", key);
        }
    }

    #[test]
    fn unknown_pair_returns_fallback() {
        let reg = MeshRegistry::new_mock();
        // No entry for (Water, Grass) if we build a minimal registry without it
        let minimal = MeshRegistryBuilder::new()
            .entry("base:missing", &[MeshTag::Flat])
            .fallback("base:missing")
            .build();

        let id = minimal.get_mesh_id(Biome::Grass, Biome::Water, 0, 0, 0);
        assert_eq!(minimal.key_for_id(id), "base:missing");
    }

    #[test]
    fn all_entries_covers_every_registered_key() {
        let reg = MeshRegistry::new_mock();
        let count = reg.all_entries().count();
        assert!(count > 0);
        // Every entry must have a namespaced key
        for e in reg.all_entries() {
            assert!(e.key.contains(':'));
        }
    }

    #[test]
    fn duplicate_entry_registration_is_idempotent() {
        let reg = MeshRegistryBuilder::new()
            .entry("base:grass_flat_a", &[MeshTag::Flat])
            .entry("base:grass_flat_a", &[MeshTag::WithFeature]) // duplicate, ignored
            .fallback("base:grass_flat_a")
            .build();

        assert_eq!(reg.all_entries().count(), 1);
    }

    #[test]
    fn spatial_hash_gives_stable_variant_selection() {
        let reg = MeshRegistry::new_mock();
        let id_a = reg.get_mesh_id(Biome::Grass, Biome::Grass, 5, 3, 1);
        let id_b = reg.get_mesh_id(Biome::Grass, Biome::Grass, 5, 3, 1);
        assert_eq!(id_a, id_b, "same inputs must always yield same variant");
    }
}