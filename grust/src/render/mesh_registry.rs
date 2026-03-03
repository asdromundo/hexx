use crate::core::terrain::Biome;
use std::collections::HashMap;

/// Tau: tau(A, B) -> MeshId
pub struct MeshRegistry {
    lut: HashMap<(Biome, Biome), Vec<i32>>,
    fallback_id: i32,
}

impl MeshRegistry {
    pub fn new(lut: HashMap<(Biome, Biome), Vec<i32>>) -> Self {
        Self {
            lut,
            fallback_id: i32::MIN,
        }
    }

    pub(crate) fn new_mock() -> Self {
        Self::new(_mock_lut())
    }

    #[inline]
    pub fn get_mesh_id(&self, inner: Biome, outer: Biome, q: i32, r: i32, d: usize) -> i32 {
        if let Some(variants) = self.lut.get(&(inner, outer)) {
            let h = spatial_hash(q, r, d);
            let index = (h as usize) % variants.len();

            return variants[index];
        }
        self.fallback_id
    }
}

fn _mock_lut() -> HashMap<(Biome, Biome), Vec<i32>> {
    let mut lut = HashMap::new();

    lut.insert((Biome::Grass, Biome::Grass), vec![0, 1]);
    lut.insert((Biome::Water, Biome::Water), vec![2, 3]);
    lut.insert((Biome::Grass, Biome::Water), vec![4, 5]);
    lut.insert((Biome::Water, Biome::Grass), vec![6, 7]);

    lut
}

// Guarantees that the memory is a contiguous block compatible with C/Godot
#[repr(C)]
pub struct HexRenderData {
    pub wedges: [i32; 6],
    pub elevation: i32,
    pub center_feature: i32,
}

#[inline(always)]
fn spatial_hash(q: i32, r: i32, d: usize) -> u32 {
    // big primes to mix the coordinates
    let mut h = (q as u32).wrapping_mul(0x9E3779B1);
    h ^= (r as u32).wrapping_mul(0x85EBCA6B);
    h ^= (d as u32).wrapping_mul(0xC2B2AE35);
    h ^= h >> 16; // Final avalanche
    h
}
