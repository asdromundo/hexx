use godot::prelude::*;
use hexx::*;
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Hex2DGrid {
    base: Base<Node2D>,
    layout: HexLayout,
    map: HashSet<Hex>,
    corners: Vec<PackedVector2Array>,
}

#[godot_api]
impl INode2D for Hex2DGrid {
    fn init(base: Base<Node2D>) -> Self {
        Hex2DGrid {
            base,
            layout: HexLayout {
                scale: Vec2::new(32.0, 32.0),
                orientation: HexOrientation::Pointy,
                origin: Vec2::new(550.0, 320.0),
            },
            map: HashSet::new(),
            corners: Vec::new(),
        }
    }

    fn ready(&mut self) {

        self.map = rect_pointy_grid(-5, 5, -5, 5);
        for hex in &self.map {
            let corners = HexLayout::hex_corners(&self.layout, *hex);
            let mut godot_corners = corners.iter().map(|c| Vector2::new(c.x, c.y)).collect::<Vec<_>>();
            godot_corners.push(godot_corners[0]); // Close the loop for drawing
            self.corners.push(PackedVector2Array::from(godot_corners));
        }
    }

    fn draw(&mut self) {
        // Fool that obscure borrow checker by temporarily taking ownership of the corners vector.
        let tmp_corners = std::mem::take(&mut self.corners);
        self.draw_hex_grid(&tmp_corners);
        // You shall not pass! (until we put the corners back)
        self.corners = tmp_corners;
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
