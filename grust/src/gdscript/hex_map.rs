use std::collections::HashSet;
use crate::gdscript::hex_utils::HexGridUtils;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct HexMap {
    base: Base<RefCounted>,
    layout: hexx::HexLayout,
    map: HashSet<hexx::Hex>,
}

#[godot_api]
impl HexMap {
    #[func]
    fn new_from(pointy_orientation: bool, p_origin: Vector2, p_scale: Vector2, limits: Vector4i) -> Gd<Self> {
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

        let map = HexGridUtils::new_rect_map(&layout, limits);
        Gd::from_init_fn(|base| Self { base, layout, map })
    }

}
