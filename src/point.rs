use p3_field::{AbstractField, Field, PrimeField64};
use p3_goldilocks::Goldilocks;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Point {
    x: Goldilocks,
    y: Goldilocks,
    is_none: bool,
}

impl Point {
    pub fn new(x: Goldilocks, y: Goldilocks) -> Self {
        Self {
            x,
            y,
            is_none: false
        }
    }

    pub fn none_point() -> Self {
        Self {
            x: Goldilocks::from_canonical_u8(0),
            y: Goldilocks::from_canonical_u8(0),
            is_none: true
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.x, self.y, self.is_none)
    }

    // pub fn to_256bit(&self) ->
}

// curve: y^2 = x^3 + Ax + B
pub fn double(a: &Point, A: Goldilocks) -> Point {
    if a.is_none {
        return Point::none_point();
    }

    //	lambda := NewFraction(3 * point.X * point.X , 2 * point.Y)
    let nom_lambda = Goldilocks::from_canonical_u8(3) * a.x * a.x + A;
    let den_lambda = Goldilocks::from_canonical_u8(2) * a.y;

    // 	fx := lambda.MulFrac(lambda).PlusInt(-2 * point.X)
    let nom_fx = nom_lambda * nom_lambda - (Goldilocks::from_canonical_u8(2) * a.x * den_lambda * den_lambda);
    let den_fx = den_lambda * den_lambda;

    // 	fy := lambda.MulFrac(fx.PlusInt(-1 * point1.X)).MulInt(-1).PlusInt(-1 * point1.Y)
    let nom_fy = - nom_lambda * (nom_fx - a.x * den_fx) - (a.y * den_lambda * den_fx);
    let den_fy = den_lambda * den_fx;

    // let inverse_x = den_fx.try_inverse();
    let inverse_x = den_fx.try_inverse();
    if inverse_x.is_none() {
        return Point::none_point();
    }
    let inverse_y = den_fy.try_inverse();
    if inverse_y.is_none() {
        return Point::none_point();
    }

    let x = nom_fx * inverse_x.unwrap();
    let y = nom_fy * inverse_y.unwrap();

    Point::new(x, y)
}

// curve: y^2 = x^3 + Ax + B
pub fn add(a: &Point, b: &Point, A: Goldilocks) -> Point {
    if a.is_none {
        return b.clone();
    }
    if b.is_none {
        return a.clone();
    }

    if *a == *b {
        return double(a, A);
    }

    // 	lambda := NewFraction(point2.Y - point1.Y, point2.X - point1.X)
    let nom_lambda = b.y - a.y;
    let den_lambda = b.x - a.x;

    // 	fx := lambda.MulFrac(lambda).PlusInt(-1 * (point1.X + point2.X))
    let nom_fx = nom_lambda * nom_lambda - (a.x + b.x) * den_lambda * den_lambda;
    let den_fx = den_lambda * den_lambda;

    // 	fy := lambda.MulFrac(fx.PlusInt(-1 * point1.X)).MulInt(-1).PlusInt(-1 * point1.Y)
    let nom_fy = -nom_lambda * (nom_fx - a.x * den_fx) - (a.y * den_lambda * den_fx);
    let den_fy = den_lambda * den_fx;

    let inverse_x = den_fx.try_inverse();
    if inverse_x.is_none() {
        return Point::none_point();
    }
    let inverse_y = den_fy.try_inverse();
    if inverse_y.is_none() {
        return Point::none_point();
    }

    let x = nom_fx * inverse_x.unwrap();
    let y = nom_fy * inverse_y.unwrap();

    Point::new(x, y)
}

pub fn mul(a: &Point, A: Goldilocks,  mut k: Goldilocks) -> Point {
    if a.is_none {
        return Point::none_point()
    }
    let mut b = a.clone();
    let mut res = Point::none_point();
    while  k != Goldilocks::zero() {
        if k.as_canonical_u64() & 1 == 1 {
            res = add(&b, &res, A);
        }
        b = double(&b, A);
        let d = k.as_canonical_u64();
        k = Goldilocks::from_canonical_u64(d / 2);
    }
    res
}

#[test]
fn test_point() {
    let a = Point::new(Goldilocks::from_canonical_u8(9), Goldilocks::from_canonical_u8(27));
    let a = Point::new(Goldilocks::from_canonical_u8(16), Goldilocks::from_canonical_u8(64));
    let r = double(&a, Goldilocks::zero());
    println!("{:?}", r);
}


