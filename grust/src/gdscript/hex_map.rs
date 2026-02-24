use crate::gdscript::hex_utils::HexGridUtils;
use godot::prelude::*;
use hexx::{ColumnMeshBuilder, Hex, MeshInfo, Vec2, Vec3};
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct HexMap {
    base: Base<RefCounted>,
    layout: hexx::HexLayout,
    map: HashSet<Hex>,
    #[var]
    scale_2d: Vector2,
    #[var]
    scale_3d: Vector3,
}

const SCALE_3D_FROM_2D: f32 = 1.0 / 32.0;

#[godot_api]
impl HexMap {
    #[func]
    fn new_from(
        pointy_orientation: bool,
        p_origin: Vector2,
        p_scale: Vector2,
        limits: Vector4i,
    ) -> Gd<Self> {
        let orientation = if pointy_orientation {
            hexx::HexOrientation::Pointy
        } else {
            hexx::HexOrientation::Flat
        };

        let origin = hexx::Vec2 {
            x: p_origin.x,
            y: p_origin.y,
        };
        let scale = hexx::Vec2 {
            x: p_scale.x,
            y: p_scale.y,
        };
        let layout = hexx::HexLayout {
            orientation,
            origin,
            scale,
        };

        let map = HexGridUtils::new_rect_map(limits);
        Gd::from_init_fn(|base| Self {
            base,
            layout,
            map,
            scale_2d: p_scale,
            scale_3d: Vector3::new(
                p_scale.x * SCALE_3D_FROM_2D,
                p_scale.y * SCALE_3D_FROM_2D,
                SCALE_3D_FROM_2D,
            ),
        })
    }

    #[func]
    fn map_to_column_mesh(&self) -> VarArray {
        let layout_3d = hexx::HexLayout {
            orientation: self.layout.orientation,
            origin: self.layout.origin,
            scale: Vec2 {
                x: self.scale_3d.x,
                y: self.scale_3d.y,
            },
        };

        let mut mesh = MeshInfo {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        };
        for hex in &self.map {
            let mesh_info = ColumnMeshBuilder::new(&layout_3d, self.scale_3d.z)
                .at(*hex)
                .facing(Vec3::Y)
                .build();
            mesh.merge_with(mesh_info);
        }
        HexGridUtils::build_surface_arrays(mesh)
    }

    #[func]
    fn map_idx_to_column_mesh(&self, idx: i32) -> VarArray {
        let layout_3d = HexGridUtils::layout2d_to_3d(&self.layout, self.scale_3d);

        let hex = self
            .map
            .iter()
            .nth(idx as usize)
            .expect("Index out of bounds");

        // let mesh_info = HexGridUtils::hex_to_column_mesh(&layout_3d, hex, self.scale_3d.z);
        let mesh_info = ColumnMeshBuilder::new(&layout_3d, self.scale_3d.z)
            .at(*hex)
            .facing(Vec3::Y)
            .build();
        godot_print!("Mesh info for hex {:?}: {:?}", hex, mesh_info);
        HexGridUtils::build_surface_arrays(mesh_info)
    }

    #[func]
    fn print_map(&self) {
        for hex in &self.map {
            godot_print!("{:?}", hex);
        }
    }
}
