use godot::prelude::*;
use hexx::HexLayout;

use crate::gdscript::hex_utils::HexGridUtils;
use crate::gdscript::map_data_ref::MapDataRef;
use crate::map::map_generator::{MapGeneratorConfig, generate};
use crate::map::mesh_registry::MeshRegistry;
use crate::rendering::hex_render_data::HexRenderDataFfi;

// ---------------------------------------------------------------------------
// MapGeneratorNode — GDExtension-exposed Node for triggering generation
// from GDScript.
//
// WHY A NODE AND NOT JUST FUNCTIONS?
//   Signals. When generation completes, this node emits `map_ready` with
//   both the MapDataRef and the Array of HexRenderDataFfi. GDScript
//   connects to the signal and drives rendering — no polling needed.
//
//   In the bevy_ecs future: this node gets replaced by a system that runs
//   generate() and inserts MapData as a Resource. The signal pattern
//   becomes a Visual Event on the queue.
// ---------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapGeneratorNode {
    base: Base<Node>,
    registry: MeshRegistry,
}

#[godot_api]
impl INode for MapGeneratorNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            registry: MeshRegistry::new_mock(),
        }
    }
}

#[godot_api]
impl MapGeneratorNode {
    #[signal]
    fn map_ready(map_data: Gd<MapDataRef>, render_data: Array<Gd<HexRenderDataFfi>>);

    /// Called from GDScript (or editor tooling) to kick off generation.
    /// `hex_map` is the existing HexMap RefCounted object.
    /// Results are emitted via the `map_ready` signal.
    #[func]
    pub fn generate_map(
        &mut self,
        seed: i64,
        pointy: bool,
        origin: Vector2,
        scale: Vector2,
        limits: Vector4i,
    ) {
        let orientation = if pointy {
            hexx::HexOrientation::Pointy
        } else {
            hexx::HexOrientation::Flat
        };

        let layout = HexLayout {
            orientation,
            origin: hexx::Vec2 { x: origin.x, y: origin.y },
            scale: hexx::Vec2 { x: scale.x, y: scale.y },
        };

        let hex_set = HexGridUtils::new_rect_map(&layout, limits);
        
        let config = MapGeneratorConfig {
            seed: seed as u64,
            hex_set,
            layout,
        };

        let output = generate(&config, &self.registry);

        // Wrap MapData for GDScript.
        let map_data_ref = MapDataRef::from_map_data(output.map_data);

        // Build the render data array for GDScript.
        let render_array: Array<Gd<HexRenderDataFfi>> = output
            .render_entries
            .iter()
            .map(|(_, rd)| HexRenderDataFfi::from_render_data(rd))
            .collect();

        // Emit — GDScript renderer connects to this signal.
        self.base_mut()
            .emit_signal("map_ready", &[map_data_ref.to_variant(), render_array.to_variant()]);
    }
}