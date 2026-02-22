/// Axial hexagonal coordinate representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hex {
    /// Q coordinate in axial system
    pub q: i32,
    /// R coordinate in axial system
    pub r: i32,
}

impl Hex {
    /// Create a new hexagonal coordinate.
    pub const fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Compute the cube-coordinate `s`.
    /// Returns: s = -q - r
    pub const fn s(&self) -> i32 {
        -self.q - self.r
    }

    /// Component-wise addition.
    pub const fn add(&self, other: Hex) -> Hex {
        Hex::new(self.q + other.q, self.r + other.r)
    }

    /// Component-wise subtraction.
    pub const fn subtract(&self, other: Hex) -> Hex {
        Hex::new(self.q - other.q, self.r - other.r)
    }

    /// Scale both coordinates by integer k.
    pub const fn scale(&self, k: i32) -> Hex {
        Hex::new(self.q * k, self.r * k)
    }

    /// Length (distance from origin) using cube coordinates formula:
    /// (|q| + |r| + |s|) / 2
    pub fn length(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s().abs()) / 2
    }

    /// Distance to another hex.
    pub fn distance(&self, other: Hex) -> i32 {
        self.subtract(other).length()
    }

    /// The six direction vectors for a hexagonal grid.
    pub const DIRECTIONS: [Hex; 6] = [
        Hex { q: 1, r: 0 },   // East
        Hex { q: 1, r: -1 },  // Southeast
        Hex { q: 0, r: -1 },  // Southwest
        Hex { q: -1, r: 0 },  // West
        Hex { q: -1, r: 1 },  // Northwest
        Hex { q: 0, r: 1 },   // Northeast
    ];
}