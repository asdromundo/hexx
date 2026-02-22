use crate::core::hex::{
    Hex,
    layout::HexLayout,
    math::{self, neighbors},
    orientation::HexOrientation,
};
use godot::prelude::*;
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Hex2DGrid {
    base: Base<Node2D>,
    hex_grid: Vec<Hex>,
    hex_grid_corners: Vec<PackedVector2Array>,
    hex_layout: HexLayout,
    map: HashSet<Hex>,
    corners: Vec<PackedVector2Array>,
}

#[godot_api]
impl INode2D for Hex2DGrid {
    fn init(base: Base<Node2D>) -> Self {
        Hex2DGrid {
            base,
            hex_grid: Vec::new(),
            hex_grid_corners: Vec::new(),
            hex_layout: HexLayout::new(
                HexOrientation::POINTY,
                Vector2::new(32.0, 32.0),
                Vector2::new(550.0, 320.0),
            ),
            map: HashSet::new(),
            corners: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let hex = Hex::new(0, 0);
        self.hex_grid.push(hex);
        self.hex_grid.extend(neighbors(hex));

        for i in 0..self.hex_grid.len() {
            let hex = self.hex_grid[i];
            let corners = math::hex_corners(&self.hex_layout, hex);
            self.hex_grid_corners
                .insert(i, PackedVector2Array::from(corners));
        }

        self.map = rect_pointy_grid(-5, 5, -5, 5);
        for hex in &self.map {
            let corners = math::hex_corners(&self.hex_layout, *hex);
            self.corners.push(PackedVector2Array::from(corners));
        }
    }

    fn draw(&mut self) {
        // Fool that obscure borrow checker by temporarily taking ownership of the corners vector.
        let tmp_corners = std::mem::take(&mut self.corners);
        self.draw_hex_grid(&tmp_corners);
        // You shall not pass! (until we put the corners back)
        self.corners = tmp_corners;

        // let corners_list = std::mem::take(&mut self.hex_grid_corners);
        // self.draw_hex_grid(&corners_list);
        // self.hex_grid_corners = corners_list;
    }
}

impl Hex2DGrid {
    fn draw_hex_grid(&mut self, corners: &Vec<PackedVector2Array>) {
        for corners in corners {
            let colors = PackedArray::<Color>::from([Color::BLUE]);

            self.base_mut().draw_polygon(&corners, &colors);

            self.base_mut().draw_polyline(&corners, Color::ALICE_BLUE);
        }
    }
}

// Axial coordinates to rectangular grid (pointy top)
fn rect_pointy_grid(left: i32, right: i32, top: i32, bottom: i32) -> HashSet<Hex> {
    let mut hexes = HashSet::new();
    for r in top..=bottom {
        // pointy top
        let r_offset = r >> 1; // or r/2
        for q in left - r_offset..=right - r_offset {
            hexes.insert(Hex::new(q, r));
        }
    }
    hexes
}
