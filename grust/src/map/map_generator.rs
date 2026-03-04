use hexx::{Hex, HexLayout};
use std::collections::HashSet;

use crate::core::terrain::Biome;
use crate::map::map_data::{MapData};
use crate::map::mesh_registry::MeshRegistry;
use crate::rendering::hex_render_data::{HexRenderData};

// ---------------------------------------------------------------------------
// MapGeneratorConfig — all inputs needed for a deterministic map gen run.
//
// WHY A CONFIG STRUCT?
//   Keeps the generator function signature stable as we add parameters.
//   In the future this becomes a bevy_ecs Resource passed into systems.
//   It also maps cleanly to your MapProfile Godot Resource (future step).
// ---------------------------------------------------------------------------

pub struct MapGeneratorConfig {
    pub seed: u64,
    pub hex_set: HashSet<Hex>,
    pub layout: HexLayout,
}

// ---------------------------------------------------------------------------
// MapGeneratorOutput — everything produced in one generation pass.
//
// Keeping both MapData and render data together means the generator is
// called once — no second pass needed. ECS gets MapData, renderer gets
// render_entries.
// ---------------------------------------------------------------------------

pub struct MapGeneratorOutput {
    /// Game state — hand this to ECS as a Resource.
    pub map_data: MapData,
    /// Per-hex render instructions — hand this to HexRenderer.gd.
    pub render_entries: Vec<(Hex, HexRenderData)>,
}

// ---------------------------------------------------------------------------
// generate — the core map generation function.
//
// This is a pure function: same config → same output, always.
// No global state, no Godot API calls, safe to run off the main thread
// (future: wrap in a bevy_ecs system or AsyncTask).
//
// BIOME ASSIGNMENT (placeholder — replace with noise later):
//   Currently uses a seeded LCG to assign biomes. The LCG is intentionally
//   simple: it's reproducible, has no external dependencies, and will be
//   replaced by a noise-based pass (e.g. FastNoiseLite seeded the same way)
//   without changing the function signature or output types.
//
// WEDGE MESH SELECTION:
//   For each hex, we look at each of its 6 neighbours. The (inner, outer)
//   biome pair for wedge d = (this_hex_biome, neighbour_d_biome).
//   If the neighbour is outside the map, we treat it as the same biome
//   (no transition at map edges — produces a clean border).
// ---------------------------------------------------------------------------

pub fn generate(config: &MapGeneratorConfig, registry: &MeshRegistry) -> MapGeneratorOutput {
    let hex_count = config.hex_set.len();

    // --- Step 1: Sort hexes for stable index assignment ---
    // Same sort order as MapData::from_unsorted, done here early so we can
    // build a lookup for neighbour queries in O(1).
    let mut sorted_hexes: Vec<Hex> = config.hex_set.iter().copied().collect();
    sorted_hexes.sort_by_key(|h| (h.x, h.y));

    // --- Step 2: Assign biomes (seeded, deterministic) ---
    let biomes: Vec<Biome> = sorted_hexes
        .iter()
        .enumerate()
        .map(|(i, _)| assign_biome(config.seed, i))
        .collect();

    // --- Step 3: Assign elevations (placeholder: all flat) ---
    let elevations: Vec<i32> = vec![0; hex_count];

    // --- Step 4: Build a fast neighbour-biome lookup ---
    // We need biome_of(neighbour_hex) during wedge mesh selection.
    // Binary search on sorted_hexes is O(log n) — acceptable for map gen
    // which runs once. Hot-path iteration uses index directly.
    let biome_of = |hex: Hex| -> Biome {
        sorted_hexes
            .binary_search_by_key(&(hex.x, hex.y), |h| (h.x, h.y))
            .ok()
            .map(|idx| biomes[idx])
            .unwrap_or_else(|| biomes[0]) // edge: treat out-of-map as first biome
    };

    // --- Step 5: Build render data per hex ---
    let render_entries: Vec<(Hex, HexRenderData)> = sorted_hexes
        .iter()
        .zip(biomes.iter())
        .zip(elevations.iter())
        .map(|((hex, inner_biome), &elevation)| {
            let neighbours = hex.all_neighbors();
            let wedge_ids: [i32; 6] = std::array::from_fn(|d| {
                let outer_biome = biome_of(neighbours[d]);
                registry.get_mesh_id(*inner_biome, outer_biome, hex.x, hex.y, d)
            });

            let world_pos = config.layout.hex_to_world_pos(*hex);

            let render_data = HexRenderData::from_ids(
                wedge_ids,
                elevation,
                None, // center_feature: not yet implemented
                world_pos,
                registry,
            );
            (*hex, render_data)
        })
        .collect();

    // --- Step 6: Assemble MapData ---
    let map_data = MapData::from_unsorted(
        sorted_hexes
            .into_iter()
            .zip(biomes.into_iter())
            .zip(elevations.into_iter())
            .map(|((hex, biome), elev)| (hex, biome, elev)),
    );

    MapGeneratorOutput { map_data, render_entries }
}

// ---------------------------------------------------------------------------
// assign_biome — deterministic biome from seed + tile index.
//
// A minimal LCG (Linear Congruential Generator). Constants from Knuth.
// This will be replaced by a noise-based pass once the pipeline is proven.
// Keeping it here (not inlined) makes it easy to swap.
// ---------------------------------------------------------------------------

fn assign_biome(seed: u64, index: usize) -> Biome {
    let state = seed
        .wrapping_add(index as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    let n = ((state >> 33) as usize) % Biome::COUNT;
    match n {
        0 => Biome::Grass,
        _ => Biome::Water,
    }
}
