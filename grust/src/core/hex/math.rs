//! Mathematical utilities for hexagonal grid operations.
//!
//! Provides functions for coordinate conversions, neighbor calculations, and geometric operations
//! on hexagonal grids using axial coordinate systems. These functions are optimized Rust
//! implementations of the corresponding GDScript HexMath class.

use godot::prelude::*;
use std::f32::consts::PI;
use crate::core::hex::Hex;
use crate::core::hex::layout::HexLayout;

/// Get the direction vector for a given direction index (0-5).
///
/// Direction indexing follows the standard hex grid convention:
/// - 0: East
/// - 1: Southeast
/// - 2: Southwest
/// - 3: West
/// - 4: Northwest
/// - 5: Northeast
///
/// # Panics
/// Panics if `dir` is not in range [0, 5].
pub fn direction(dir: usize) -> Hex {
    debug_assert!(dir < 6, "Direction index must be in range [0, 5]");
    Hex::DIRECTIONS[dir]
}

/// Get all six neighbors of a hexagon.
pub fn neighbors(hex: Hex) -> Vec<Hex> {
    Hex::DIRECTIONS.iter().map(|&dir| hex.add(dir)).collect()
}

/// Convert hexagonal coordinates to pixel coordinates.
///
/// Uses the orientation and size information from the HexLayout to transform
/// axial hex coordinates into a 2D point in pixel space.
pub fn hex_to_point2d(layout: &HexLayout, hex: Hex) -> Vector2 {
    let m = &layout.orientation;
    let x = (m.f0 * hex.q as f32 + m.f1 * hex.r as f32) * layout.size.x;
    let y = (m.f2 * hex.q as f32 + m.f3 * hex.r as f32) * layout.size.y;
    Vector2::new(x + layout.origin.x, y + layout.origin.y)
}

/// Round floating-point axial coordinates to the nearest valid hex.
///
/// Uses the axial rounding algorithm to find the nearest hex coordinate
/// from potentially fractional values.
pub fn axial_round(x: f32, y: f32) -> (i32, i32) {
    let xgrid = x.round() as i32;
    let ygrid = y.round() as i32;

    let xf = x - xgrid as f32;
    let yf = y - ygrid as f32;

    let mx = (xf.abs() >= yf.abs()) as i32;
    let my = 1 - mx;

    let rx = (xf + 0.5 * yf).round() as i32;
    let ry = (yf + 0.5 * xf).round() as i32;

    (
        xgrid + mx * rx,
        ygrid + my * ry,
    )
}

/// Convert pixel coordinates to the nearest hexagonal coordinate.
///
/// Transforms a 2D point in pixel space back into axial hex coordinates,
/// rounding to the nearest valid hex.
pub fn point2d_to_hex(layout: &HexLayout, p: Vector2) -> Hex {
    let m = &layout.orientation;
    let pt = Vector2::new(
        (p.x - layout.origin.x) / layout.size.x,
        (p.y - layout.origin.y) / layout.size.y,
    );
    
    let q = m.b0 * pt.x + m.b1 * pt.y;
    let r = m.b2 * pt.x + m.b3 * pt.y;
    
    let (qr, rr) = axial_round(q, r);
    Hex::new(qr, rr)
}

/// Calculate the pixel offset for a corner of a hexagon.
///
/// Computes the offset vector from the hex center to a specific corner,
/// taking into account the layout's orientation.
///
/// # Arguments
/// * `layout` - The hexagonal grid layout
/// * `corner` - The corner index (0-5)
pub fn hex_corner_offset(layout: &HexLayout, corner: usize) -> Vector2 {
    let angle = 2.0 * PI * (layout.orientation.start_angle - corner as f32) / 6.0;
    Vector2::new(
        layout.size.x * angle.cos(),
        layout.size.y * angle.sin(),
    )
}

/// Get all corners of a hexagon as pixel coordinates.
///
/// Generates the 6 corner positions for a hexagon in pixel space.
/// The result includes a 7th element that repeats the first corner to close the polygon.
pub fn hex_corners(layout: &HexLayout, hex: Hex) -> [Vector2; 7] {
    let mut corners = [Vector2::ZERO; 7];
    let center = hex_to_point2d(layout, hex);
    for i in 0..6 {
        let offset = hex_corner_offset(layout, i);
        corners[i] = center + offset;
    }
    
    // Close the loop
    corners[6] = corners[0];
    
    corners
}

#[derive(GodotClass)]
#[class(init, base=Object)]
struct RHexMath;

#[godot_api]
impl RHexMath {
    #[func]
    pub fn axial_round(x: f32, y: f32) -> Vector2i {
        let (q, r) = axial_round(x, y);
        Vector2i::new(q, r)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_creation() {
        let h = Hex::new(1, 2);
        assert_eq!(h.q, 1);
        assert_eq!(h.r, 2);
        assert_eq!(h.s(), -3);
    }

    #[test]
    fn test_hex_distance() {
        let h1 = Hex::new(0, 0);
        let h2 = Hex::new(1, 0);
        assert_eq!(h1.distance(h2), 1);
    }

    #[test]
    fn test_neighbors() {
        let h = Hex::new(0, 0);
        let n = neighbors(h);
        assert_eq!(n.len(), 6);
    }

    #[test]
    fn test_axial_round() {
        let (q, r) = axial_round(1.1, 2.1);
        assert_eq!(q, 1);
        assert_eq!(r, 2);
    }
}
