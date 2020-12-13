use float_cmp::ApproxEq;
use num_traits::{float::Float, FromPrimitive};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Tuple<F> {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

pub fn cross<F: Copy + Clone + Float + FromPrimitive + Mul<Output = F>>(
    a: &Tuple<F>,
    b: &Tuple<F>,
) -> Tuple<F> {
    if !a.is_vector() || !b.is_vector() {
        panic!("cross product on non-vector")
    }

    Tuple::vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

pub fn dot<F: Copy + Clone + Add<Output = F> + Mul<Output = F>>(a: &Tuple<F>, b: &Tuple<F>) -> F {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

impl<F: Float + FromPrimitive> Tuple<F> {
    pub fn point(x: F, y: F, z: F) -> Self {
        Self::new(x, y, z, F::from_f64(1.0).unwrap())
    }

    pub fn vector(x: F, y: F, z: F) -> Self {
        Self::new(x, y, z, F::from_f64(0.0).unwrap())
    }

    pub fn is_point(&self) -> bool {
        self.w == F::from_f64(1.0).unwrap()
    }

    pub fn is_vector(&self) -> bool {
        self.w == F::from_f64(0.0).unwrap()
    }

    pub fn magnitude(&self) -> F {
        let two = F::from_f64(2.0).unwrap();
        (self.x.powf(two) + self.y.powf(two) + self.z.powf(two) + self.w.powf(two)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Add<Output = T>> Add for Tuple<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Tuple<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: Clone + Copy + Mul<Output = T>> Mul<T> for Tuple<T> {
    type Output = Tuple<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T: Clone + Copy + Div<Output = T>> Div<T> for Tuple<T> {
    type Output = Tuple<T>;

    fn div(self, rhs: T) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T: Default + Neg<Output = T> + Sub<Output = T>> Neg for Tuple<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::default() - self
    }
}

impl<'a, M: Copy + Default, F: Copy + ApproxEq<Margin = M>> ApproxEq for &'a Tuple<F> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
            && self.w.approx_eq(other.w, margin)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::F64Margin;

    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(a.approx_eq(
            &Tuple {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1.0,
            },
            F64Margin::default(),
        ));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(a.approx_eq(
            &Tuple {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 0.0
            },
            F64Margin::default()
        ));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);

        assert!(p.approx_eq(
            &Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            },
            F64Margin::default()
        ));
    }

    #[test]
    fn vector_creates_tuples_with_w_1() {
        let p = Tuple::vector(4.0, -4.0, 3.0);

        assert!(p.approx_eq(
            &Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            },
            F64Margin::default()
        ));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert!((a1 + a2).approx_eq(
            &Tuple {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            },
            F64Margin::default()
        ));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert!((p1 - p2).approx_eq(&Tuple::vector(-2.0, -4.0, -6.0), F64Margin::default()));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert!((p - v).approx_eq(&Tuple::point(-2.0, -4.0, -6.0), F64Margin::default()));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert!((v1 - v2).approx_eq(&Tuple::vector(-2.0, -4.0, -6.0), F64Margin::default()));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert!((zero - v).approx_eq(&Tuple::vector(-1.0, 2.0, -3.0), F64Margin::default()));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert!((-a).approx_eq(&Tuple::new(-1.0, 2.0, -3.0, 4.0), F64Margin::default()));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert!((a * 3.5).approx_eq(&Tuple::new(3.5, -7.0, 10.5, -14.0), F64Margin::default()));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert!((a * 0.5).approx_eq(&Tuple::new(0.5, -1.0, 1.5, -2.0), F64Margin::default()));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert!((a / 2.0).approx_eq(&Tuple::new(0.5, -1.0, 1.5, -2.0), F64Margin::default()));
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);

        assert!(v.magnitude().approx_eq(1.0, F64Margin::default()));
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);

        assert!(v.magnitude().approx_eq(1.0, F64Margin::default()));
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);

        assert!(v.magnitude().approx_eq(1.0, F64Margin::default()));
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert!(v.magnitude().approx_eq(14f64.sqrt(), F64Margin::default()));
    }

    #[test]
    fn magnitude_of_vector_neg_1_2_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);

        assert!(v.magnitude().approx_eq(14f64.sqrt(), F64Margin::default()));
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        assert!(v
            .normalized()
            .approx_eq(&Tuple::vector(1.0, 0.0, 0.0), F64Margin::default()));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert!(v.normalized().approx_eq(
            &Tuple::vector(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt()),
            F64Margin::default()
        ));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let norm = v.normalized();

        assert!(norm.magnitude().approx_eq(1.0, F64Margin::default()));
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert!(dot(&a, &b).approx_eq(20.0, F64Margin::default()));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert!(cross(&a, &b).approx_eq(&Tuple::vector(-1.0, 2.0, -1.0), F64Margin::default()));
        assert!(cross(&b, &a).approx_eq(&Tuple::vector(1.0, -2.0, 1.0), F64Margin::default()));
    }
}
