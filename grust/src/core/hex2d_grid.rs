use crate::core::hex::{
    Hex,
    layout::HexLayout,
    math::{self, neighbors},
    orientation::HexOrientation,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Hex2DGrid {
    base: Base<Node2D>,
    hex_grid: Vec<Hex>,
    hex_grid_corners: Vec<PackedVector2Array>,
    hex_layout: HexLayout,
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
    }

fn draw(&mut self) {
    // 1. Extraemos el Vec de self. En su lugar queda un Vec vacío de forma temporal.
    // Esto "libera" a self para que base_mut() pueda usarlo sin problemas.
    let corners_list = std::mem::take(&mut self.hex_grid_corners);

    // 2. Iteramos sobre la lista que acabamos de extraer (self ya no está prestado aquí)
    for corners in &corners_list {
        // 3. Ahora podemos llamar a base_mut() sin conflictos
        let colors = PackedArray::<Color>::from([Color::BLUE]);

        self.base_mut()
            .draw_polygon(corners, &colors);

        self.base_mut()
            .draw_polyline(corners, Color::ALICE_BLUE);

    }

    // 4. ¡Muy importante! Devolvemos la lista a su lugar original en el struct
    self.hex_grid_corners = corners_list;
}
}
