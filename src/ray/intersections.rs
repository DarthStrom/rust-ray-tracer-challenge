#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Self { t, object }
    }
}

#[derive(Debug, Default)]
pub struct Intersections<'a> {
    pub data: Vec<Intersection<'a>>,
}

use std::{iter::FromIterator, ops::Index};

use crate::sphere::Sphere;

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        Self {
            data: intersections,
        }
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        self.data
            .iter()
            .filter(|i| i.t >= 0.0)
            .min_by(|&i1, &i2| i1.t.partial_cmp(&i2.t).unwrap())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn append(&mut self, other: &mut Intersections<'a>) {
        self.data.append(&mut other.data);
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> FromIterator<Intersection<'a>> for Intersections<'a> {
    fn from_iter<T: IntoIterator<Item = Intersection<'a>>>(iter: T) -> Self {
        let mut result = Self { data: vec![] };
        for i in iter {
            result.data.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = Intersection::new(3.5, &s);

        f_assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        f_assert_eq!(xs[0].t, 1.0);
        f_assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }
}
