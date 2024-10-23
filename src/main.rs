use p3_field::AbstractField;
use p3_goldilocks::Goldilocks;
use crate::point::Point;

mod point;
mod utils;
mod curve;
mod ecdh;
mod errors;

const P: u64 = 0xFFFF_FFFF_0000_0001;

fn main() {
    let point = Point::new(Goldilocks::from_canonical_u64(3), Goldilocks::from_canonical_u64(4));
    println!("{}", point.to_string());
}
