use crate::tuple::Tuple;
use num_traits::{Float, FromPrimitive};
use std::iter::Sum;
use std::ops::{Add, Index, Mul};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<F: Float + FromPrimitive> Matrix<F> {
    pub fn new(data: Vec<Vec<F>>) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        let one = F::from_f64(1.0).unwrap();
        let zero = F::from_f64(0.0).unwrap();
        Self {
            data: vec![
                vec![one, zero, zero, zero],
                vec![zero, one, zero, zero],
                vec![zero, zero, one, zero],
                vec![zero, zero, zero, one],
            ],
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<T: Copy + Clone + Sum + Mul<Output = T>> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut data = vec![];
        for y in 0..self.data.len() {
            let mut row = vec![];
            for x in 0..self.data[0].len() {
                row.push(
                    self[y]
                        .iter()
                        .enumerate()
                        .map(|(i, val)| *val * rhs[i][x])
                        .sum(),
                );
            }
            data.push(row);
        }
        Self { data }
    }
}

impl<F: Float + FromPrimitive + Mul<Output = F> + Add<Output = F>> Mul<Tuple<F>> for Matrix<F> {
    type Output = Tuple<F>;

    fn mul(self, rhs: Tuple<F>) -> Self::Output {
        Tuple::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn a_2x2_matrix() {
        let m = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn a_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn equal_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn unequal_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);

        assert!(a != b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            (a * b),
            Matrix::new(vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ])
        )
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiplying_by_identity_matrix() {
        let a = Matrix::new(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(a.clone() * Matrix::identity(), a);
    }
}
