use p3_field::AbstractField;
use p3_goldilocks::Goldilocks;
use crate::point::{mul, Point};


// y^2 = x^3 + ax + b
pub struct Elliptic {
    g: Point,
    a: Goldilocks,
    b: Goldilocks,
}

impl Elliptic {
    pub fn new(g_x: Goldilocks, g_y: Goldilocks, a: Goldilocks, b: Goldilocks) -> Self{
        let g = Point::new(g_x, g_y);
        Self {
            g,
            a,
            b
        }
    }

    // Generate returns (x`, y`) which is a k multiplication of G (x, y) on the elliptic curve e.
    pub fn generate(&self, k: Goldilocks) -> Point {
        mul(&self.g, self.a, k)
    }

    // Calculate returns (x`, y`) which is a k multiplication of (x, y) on the elliptic curve e.
    pub fn calculate(&self, xy: &Point, k: Goldilocks) -> Point {
        mul(xy, self.a, k)
    }

    pub fn sample() -> Self {
        Self {
            g: Point::new(Goldilocks::zero(),Goldilocks::from_canonical_u64(3)),
            a: Goldilocks::zero(),
            b: Goldilocks::from_canonical_u64(9)
        }
    }
}

