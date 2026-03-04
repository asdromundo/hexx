use std::collections::HashSet;

use godot::classes::mesh::ArrayType;
use godot::obj::IndexEnum;
use godot::prelude::*;
use hexx::{HexLayout, MeshInfo, Vec2};

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct HexGridUtils;

#[godot_api]
impl HexGridUtils {}

impl HexGridUtils {
    pub(crate) fn build_surface_arrays(mesh_info: MeshInfo) -> VarArray {
        let vertices: PackedVector3Array = mesh_info
            .vertices
            .iter()
            .map(|v| Vector3::new(v.x, v.y, v.z))
            .collect();
        let normals: PackedVector3Array = mesh_info
            .normals
            .iter()
            .map(|n| Vector3::new(n.x, n.y, n.z))
            .collect();
        let uvs: PackedVector2Array = mesh_info
            .uvs
            .iter()
            .map(|uv| Vector2::new(uv.x, uv.y))
            .collect();
        // let indices: PackedInt32Array = mesh_info.indices.iter().map(|i| *i as i32).collect();
        let indices: PackedInt32Array = mesh_info
            .indices
            .chunks_exact(3)
            // Recibimos un slice con [v1, v2, v3] y devolvemos [v1, v3, v2]
            .flat_map(|chunk| [chunk[0] as i32, chunk[2] as i32, chunk[1] as i32])
            .collect();

        let mut arrays = VarArray::new();
        arrays.resize(ArrayType::MAX.to_index(), &Variant::nil());

        arrays.set(ArrayType::VERTEX.to_index(), &vertices.to_variant());
        arrays.set(ArrayType::NORMAL.to_index(), &normals.to_variant());
        arrays.set(ArrayType::TEX_UV.to_index(), &uvs.to_variant());
        arrays.set(ArrayType::INDEX.to_index(), &indices.to_variant());

        arrays
    }

    /// Creates a rectangular hex map based on the provided layout and limits.
    /// The limits are defined as (left, right, top, down) and will be adjusted based on the hex orientation.
    /// This is based on the RedBlob Games article: https://www.redblobgames.com/grids/hexagons/implementation.html#shape-rectangle
    pub(crate) fn new_rect_map(layout: &HexLayout, offset: Vector4i) -> HashSet<hexx::Hex> {
        match layout.orientation {
            hexx::HexOrientation::Pointy => {
                // let mut map = HashSet::new();
                // for r in offset.z..=offset.w {
                //     let r_offset = r >> 1; // r_offset = floor(r/2)
                //     for q in offset.x - r_offset..=offset.y - r_offset {
                //         map.insert(hexx::hex(q, r));
                //     }
                // }
                // map

                // Functional implementation of the above nested loops:
                (offset.z..=offset.w)
                    .flat_map(|r| {
                        let r_offset = r >> 1; // r_offset = floor(r/2)
                        (offset.x - r_offset..=offset.y - r_offset).map(move |q| hexx::hex(q, r))
                    })
                    .collect::<HashSet<hexx::Hex>>()
            }
            hexx::HexOrientation::Flat => {
                // let mut map = HashSet::new();
                // for q in offset.x..=offset.y {
                //     let q_offset = q >> 1; // r_offset = floor(q/2)
                //     for r in offset.z - q_offset..=offset.w - q_offset {
                //         map.insert(hexx::hex(q, r));
                //     }
                // }
                // map

                (offset.x..=offset.y)
                    .flat_map(|q| {
                        let q_offset = q >> 1; // r_offset = floor(q/2)
                        (offset.z - q_offset..=offset.w - q_offset).map(move |r| hexx::hex(q, r))
                    })
                    .collect::<HashSet<hexx::Hex>>()
            }
        }
    }

    pub(crate) fn layout2d_to_3d(layout: &HexLayout, scale_3d: Vector3) -> HexLayout {
        HexLayout {
            orientation: layout.orientation,
            origin: layout.origin,
            scale: Vec2 {
                x: scale_3d.x,
                y: scale_3d.y,
            },
        }
    }
}
