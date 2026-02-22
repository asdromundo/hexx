use godot::prelude::*;
use crate::core::hex::orientation::HexOrientation;

/// Hexagonal grid layout configuration.
/// Represents the orientation, size, and origin of a hexagonal grid.
#[derive(Debug, Clone)]
pub struct HexLayout {
    /// Orientation of the hexagons (pointy-top or flat-top)
    pub orientation: HexOrientation,
    /// Size of the hexagons as (width, height)
    pub size: Vector2,
    /// Origin point for coordinate conversions
    pub origin: Vector2,
}

impl HexLayout {
    /// Create a new hex layout.
    pub fn new(orientation: HexOrientation, size: Vector2, origin: Vector2) -> Self {
        Self {
            orientation,
            size,
            origin,
        }
    }
}