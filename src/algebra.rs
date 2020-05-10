extern crate derive_more;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::Mul;

// Point2f is also Vec2f
// 2f means 2d+f32
#[derive(
    Copy,
    Clone,
    PartialEq,
    Debug,
    Default,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Point2f {
    // / x \
    // \ y /
    pub x: f32,
    pub y: f32,
}

impl Point2f {
    pub fn new() -> Point2f {
        Point2f { x: 0., y: 0. }
    }

    pub fn from_floats(x: f32, y: f32) -> Point2f {
        Point2f { x, y }
    }

    pub fn from_polar(r: f32, theta: f32) -> Point2f {
        Point2f {
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    pub fn from_theta(theta: f32) -> Point2f {
        Point2f {
            x: theta.cos(),
            y: theta.sin(),
        }
    }

    pub fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normed(self) -> Point2f {
        self / self.norm()
    }

    pub fn dotx(self, other: Point2f) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn crossx(self, other: Point2f) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Mul<Point2f> for Point2f {
    type Output = Point2f;

    fn mul(self, other: Point2f) -> Point2f {
        Point2f {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

#[derive(
    Copy,
    Clone,
    PartialEq,
    Debug,
    Default,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Circle2f {
    pub center: Point2f,
    pub r: f32,
}

impl Circle2f {
    pub fn from_floats(x: f32, y: f32, r: f32) -> Circle2f {
        Circle2f {
            center: Point2f::from_floats(x, y),
            r,
        }
    }
}

// test if ab cross cd(including end point)
pub fn linesegs_distance(a: Point2f, b: Point2f, c: Point2f, d: Point2f) -> f32 {
    // strict check only
    let ab = b - a;
    let ac = c - a;
    let ad = d - a;
    let bc = c - b;
    let cd = d - c;
    if ab.crossx(ac) * ab.crossx(ad) < 0. && cd.crossx(ac) * cd.crossx(bc) < 0. {
        0.
    } else {
        let bd = d - b;
        let ab_norm = ab.norm();
        let cd_norm = cd.norm();
        let ac_norm = ac.norm();
        let ad_norm = ad.norm();
        let bc_norm = bc.norm();
        let bd_norm = bd.norm();
        let mut possible_value = vec![ac_norm, ad_norm, bc_norm, bd_norm];

        let tmp = ab.dotx(ac) / ab_norm;
        if tmp > 0. && tmp < ab_norm {
            // is using sqrt safe here?
            possible_value.push((ac_norm * ac_norm - tmp * tmp).sqrt())
        }

        let tmp = ab.dotx(ad) / ab_norm;
        if tmp > 0. && tmp < ab_norm {
            possible_value.push((ad_norm * ad_norm - tmp * tmp).sqrt())
        }

        let tmp = cd.dotx(ac) / cd_norm;
        if tmp < 0. && tmp > -cd_norm {
            possible_value.push((ac_norm * ac_norm - tmp * tmp).sqrt())
        }

        let tmp = cd.dotx(bc) / cd_norm;
        if tmp < 0. && tmp > -cd_norm {
            possible_value.push((bc_norm * bc_norm - tmp * tmp).sqrt())
        }

        possible_value
            .iter()
            .fold(f32::INFINITY, |min, x| min.min(*x))
    }
}

#[derive(
    Copy,
    Clone,
    PartialEq,
    Debug,
    Default,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
)]
pub struct Mat2x2f {
    // / x1 x2 \
    // \ y1 y2 /
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl Mul<Point2f> for Mat2x2f {
    type Output = Point2f;

    fn mul(self, rhs: Point2f) -> Point2f {
        Point2f {
            x: self.x1 * rhs.x + self.x2 * rhs.y,
            y: self.y1 * rhs.x + self.y2 * rhs.y,
        }
    }
}

impl Mat2x2f {
    pub fn from_theta(theta: f32) -> Mat2x2f {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        Mat2x2f {
            x1: cos_theta,
            x2: -sin_theta,
            y1: sin_theta,
            y2: cos_theta,
        }
    }

    pub fn from_normed_vec2f(direction: Point2f) -> Mat2x2f {
        Mat2x2f {
            x1: direction.x,
            x2: -direction.y,
            y1: direction.y,
            y2: direction.x,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect2f {
    pub lu: Point2f,
    pub rd: Point2f,
}

impl Rect2f {
    pub fn get_size(&self) -> Point2f {
        Point2f::from_floats(self.rd.x - self.lu.x, self.rd.y - self.lu.y)
    }

    pub fn from_floats(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect2f {
        Rect2f {
            lu: Point2f::from_floats(x1, y1),
            rd: Point2f::from_floats(x2, y2),
        }
    }

    // check if point2f falls inside the rectangle(not falls on)
    pub fn contain(&self, point2f: Point2f) -> bool {
        point2f.x > self.lu.x
            && point2f.x < self.rd.x
            && point2f.y > self.lu.y
            && point2f.y < self.rd.y
    }

    // find the nearest point in the rectangle to a given point
    pub fn nearest(&self, point2f: Point2f) -> Point2f {
        let mut nearest_point = Point2f::new();
        if point2f.x < self.lu.x {
            nearest_point.x = self.lu.x;
        } else if point2f.x > self.rd.x {
            nearest_point.x = self.rd.x;
        } else {
            nearest_point.x = point2f.x;
        }
        if point2f.y < self.lu.y {
            nearest_point.y = self.lu.y;
        } else if point2f.y > self.rd.y {
            nearest_point.y = self.rd.y;
        } else {
            nearest_point.y = point2f.y;
        }
        nearest_point
    }
}

#[cfg(test)]
mod test {
    use super::{linesegs_distance, Mat2x2f, Point2f};

    #[test]
    fn test_point2f_derive_more() {
        let mut point2f = Point2f::from_floats(1.0, 1.0);
        // PartialEq
        assert_eq!(point2f, Point2f::from_floats(1.0, 1.0));

        // AddAssign
        point2f += Point2f::from_floats(1.0, 1.0);
        assert_eq!(point2f, Point2f::from_floats(2.0, 2.0));

        // MulAssign
        point2f *= 2.;
        assert_eq!(point2f, Point2f::from_floats(4.0, 4.0));
    }

    #[test]
    fn test_mat2x2f() {
        let mat2x2f = Mat2x2f::from_theta(std::f32::consts::PI / 2.);
        let eps: f32 = 1e-6;
        // / 0 -1 \
        // \ 1  0 /
        assert!(mat2x2f.x1.abs() < eps);
        assert!((mat2x2f.x2 + 1.).abs() < eps);
        assert!((mat2x2f.y1 - 1.).abs() < eps);
        assert!(mat2x2f.y2.abs() < eps);

        // Rotate
        let point2f = Point2f::from_floats(3.0, 4.0);
        let point2f = mat2x2f * point2f;
        assert!((point2f.x + 4.).abs() < eps);
        assert!((point2f.y - 3.).abs() < eps);
    }

    #[test]
    fn test_crossx() {
        let eps: f32 = 1e-6;
        let p1 = Point2f::from_floats(1.0, 2.0);
        let p2 = Point2f::from_floats(-2.0, 1.0);
        assert!((p1.crossx(p2) - 5.0).abs() < eps)
    }

    #[test]
    fn test_linesegs_distance() {
        let eps: f32 = 1e-5;
        macro_rules! t {
            //test_intersection_from_8_floats
            ($x1: expr, $y1: expr,
                $x2: expr, $y2: expr,
                $x3: expr, $y3: expr,
                $x4: expr, $y4: expr,
                $expect: expr) => {
                let a = Point2f::from_floats($x1, $y1);
                let b = Point2f::from_floats($x2, $y2);
                let c = Point2f::from_floats($x3, $y3);
                let d = Point2f::from_floats($x4, $y4);
                let result = linesegs_distance(a, b, c, d);
                println!("expect {} = {}", result, $expect);
                assert!((result - $expect).abs() < eps);
            };
        }
        t!(0., 0., 1., 1., 0., 1., 1., 0., 0.); //X
        t!(0., -0.1, 0., 1., -0.1, 0., 1., 0., 0.); //L(cross)
        t!(0., 0.1, 0., 1., 0., 0., 1., 0., 0.1); //L(not cross)
        t!(0., 0., 0., 1., 1., 0., 1., 1., 1.); //||
        t!(0., 0., 0., 1., 0., 1., 0., 2., 0.); //--
        t!(0., 0., 0., 1., 0., 2., 0., 3., 1.); //- -
        t!(0., 0., 0., 3., 0., 1., 0., 2., 0.); //long overlap
        t!(0., 0., 0., 3., 0.1, 1., 0.1, 2., 0.1); // almost long overlap
        t!(247.0, 249.90126, 247.0, 282.8828, 250.0, 269.59827, 250.0, 268.93863, 3.);
    }
}
