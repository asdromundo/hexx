use crate::gdscript::hex_utils::HexGridUtils;
use godot::{
    classes::{ArrayMesh, MeshInstance3D, StandardMaterial3D, mesh::PrimitiveType},
    prelude::*,
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout, HexOrientation, Vec2, Vec3};
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Hex3DGrid {
    base: Base<Node3D>,
    layout: HexLayout,
    map: HashSet<Hex>,
    corners: Vec<PackedVector2Array>,
}

#[godot_api]
impl INode3D for Hex3DGrid {
    fn init(base: Base<Node3D>) -> Self {
        Hex3DGrid {
            base,
            layout: HexLayout {
                scale: Vec2::new(320.0, 320.0),
                orientation: HexOrientation::Pointy,
                origin: Vec2::ZERO,
            },
            map: HashSet::new(),
            corners: Vec::new(),
        }
    }

    fn ready(&mut self) {
        godot_print!("Hex3DGrid ready");
        self.create_hex_mesh();
    }
}

#[godot_api]
impl Hex3DGrid {
    #[func]
    pub fn create_hex_mesh(&mut self) {
        let layout = HexLayout::default();
        let mesh_info = ColumnMeshBuilder::new(&layout, 1.0)
            .at(Hex::new(0, 0))
            .facing(Vec3::Y)
            // .with_subdivisions(1)
            .build();

        let arr = HexGridUtils::build_surface_arrays(mesh_info);

        godot_print!("Surface array: {:?}", arr);
        let mut arr_mesh = ArrayMesh::new_gd();
        arr_mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, &arr);

        let mut mat = StandardMaterial3D::new_gd();
        mat.set_albedo(Color::RED);
        arr_mesh.surface_set_material(0, &mat);

        let mut mesh_instace = MeshInstance3D::new_alloc();
        mesh_instace.set_mesh(&arr_mesh);
        mesh_instace.set_scale(Vector3::ONE);

        self.base_mut().add_child(&mesh_instace);
    }

    // fn add_mesh_to_scene(&mut self, mesh: MeshInstance3D) {
    //     self.base_mut().add_child(&mesh);
    // }
    //     #[func]
    //     pub fn create_hex_mesh(&self) -> Gd<ArrayMesh> {
    //         // 1. Configurar hexx y generar la malla pura
    //         let layout = HexLayout::default();

    //         // Generamos la información de la malla (MeshInfo) usando un builder de hexx.
    //         // Aquí creamos una columna hexagonal (útil para 3D).
    //         let mesh_info = ColumnMeshBuilder::new(&layout, 1.0)
    //             .at(Hex::ZERO)
    //             .build();

    //         // 2. Preparar los arreglos optimizados de Godot
    //         let mut vertices = PackedVector3Array::new();
    //         let mut normals = PackedVector3Array::new();
    //         let mut uvs = PackedVector2Array::new();
    //         let mut indices = PackedInt32Array::new();

    //         // 3. Traducir de hexx (glam) a Godot
    //         for v in mesh_info.vertices {
    //             // IMPORTANTE: Godot usa un sistema Y-Up. Dependiendo de cómo configures
    //             // tu layout en hexx, podrías necesitar intercambiar ejes (ej: Vector3::new(v.x, v.z, -v.y))
    //             vertices.push(Vector3::new(v.x, v.y, v.z));
    //         }

    //         for n in mesh_info.normals {
    //                 normals.push(Vector3::new(n.x, n.y, n.z));
    //             }

    //          for uv in mesh_info.uvs {
    //                 uvs.push(Vector2::new(uv.x, uv.y));
    //             }

    //         for i in mesh_info.indices {
    //             // Godot requiere enteros de 32 bits para los índices
    //             indices.push(i as i32);
    //         }

    //         // 4. Ensamblar el Array de Superficie (Surface Array)
    //         let mut surface_array = Array::<Variant>::new();

    //         // Godot exige que el arreglo tenga un tamaño específico lleno de 'nil' inicialmente
    //         for _ in 0..ArrayType::MAX as usize {
    //             surface_array.push(Variant::nil());
    //         }

    //         surface_array.set(ArrayType::VERTEX as usize, vertices.to_variant());
    //         surface_array.set(ArrayType::INDEX as usize, indices.to_variant());

    //         if !normals.is_empty() {
    //             surface_array.set(ArrayType::NORMAL as usize, normals.to_variant());
    //         }
    //         if !uvs.is_empty() {
    //             surface_array.set(ArrayType::TEX_UV as usize, uvs.to_variant());
    //         }

    //         // 5. Instanciar el ArrayMesh en Godot
    //         let mut array_mesh = ArrayMesh::new_gd();
    //         array_mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, surface_array);

    //         array_mesh
    //     }
}
