/// Orientation configuration for hexagons.
/// Contains forward and backward transformation matrices for coordinate conversion.
#[derive(Debug, Clone, Copy)]
pub struct HexOrientation {
    /// Forward transformation matrix components
    pub f0: f32,
    pub f1: f32,
    pub f2: f32,
    pub f3: f32,
    /// Backward transformation matrix components
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub b3: f32,
    /// Starting angle for corner calculations
    pub start_angle: f32,
}

impl HexOrientation {
    /// Pointy-top hexagon orientation (flat edges on sides).
    pub const POINTY: HexOrientation = HexOrientation {
        f0: 1.732_050_8,      // sqrt(3)
        f1: 0.866_025_4,      // sqrt(3)/2
        f2: 0.0,
        f3: 1.5,
        b0: 0.577_350_3,      // sqrt(3)/3
        b1: -0.333_333_3,     // -1/3
        b2: 0.0,
        b3: 0.666_666_7,      // 2/3
        start_angle: 0.5,
    };

    /// Flat-top hexagon orientation (flat edges on top/bottom).
    pub const FLAT: HexOrientation = HexOrientation {
        f0: 1.5,
        f1: 0.0,
        f2: 0.866_025_4,      // sqrt(3)/2
        f3: 1.732_050_8,      // sqrt(3)
        b0: 0.666_666_7,      // 2/3
        b1: 0.0,
        b2: -0.333_333_3,     // -1/3
        b3: 0.577_350_3,      // sqrt(3)/3
        start_angle: 0.0,
    };
}