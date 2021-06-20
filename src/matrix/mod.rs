use crate::tuple::Tuple;
use float_cmp::{ApproxEq, F64Margin};
use std::ops::IndexMut;
use std::ops::{Index, Mul};

pub mod transform;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.data.len() == 2 && self.data[0].len() == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            self.data[0]
                .iter()
                .enumerate()
                .map(|(i, &x)| x * self.cofactor(0, i))
                .sum()
        }
    }

    pub fn identity() -> Self {
        Self {
            data: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn inverse(&self) -> Result<Self, String> {
        if !self.is_invertible() {
            return Err("not invertible".to_string());
        }

        let mut new_matrix = vec![];
        for _ in 0..self.data.len() {
            new_matrix.push(self.data[0].clone());
        }
        for (i, row) in self.data.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                new_matrix[j][i] = self.cofactor(i, j) / self.determinant();
            }
        }

        Ok(Self { data: new_matrix })
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        Self {
            data: self
                .data
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != row)
                .map(|(_, r)| {
                    let mut new_row = r.clone();
                    new_row.remove(col);
                    new_row
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn transpose(&self) -> Self {
        let mut result = vec![];
        for _ in 0..self[0].len() {
            result.push(vec![]);
        }
        for y in self.data.iter() {
            for (i, &x) in y.iter().enumerate() {
                result[i].push(x)
            }
        }
        Self { data: result }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Vec<f64> {
        &mut self.data[i]
    }
}

impl Mul for Matrix {
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

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w,
        )
    }
}

impl<'a> ApproxEq for &'a Matrix {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.data.iter().enumerate().all(|(i, r)| {
            r.iter()
                .enumerate()
                .all(|(j, x)| x.approx_eq(other[i][j], margin))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{tuple::Tuple, MARGIN};

    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(m[0][0].approx_eq(1.0, MARGIN));
        assert!(m[0][3].approx_eq(4.0, MARGIN));
        assert!(m[1][0].approx_eq(5.5, MARGIN));
        assert!(m[1][2].approx_eq(7.5, MARGIN));
        assert!(m[2][2].approx_eq(11.0, MARGIN));
        assert!(m[3][0].approx_eq(13.5, MARGIN));
        assert!(m[3][2].approx_eq(15.5, MARGIN));
    }

    #[test]
    fn a_2x2_matrix() {
        let m = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert!(m[0][0].approx_eq(-3.0, MARGIN));
        assert!(m[0][1].approx_eq(5.0, MARGIN));
        assert!(m[1][0].approx_eq(1.0, MARGIN));
        assert!(m[1][1].approx_eq(-2.0, MARGIN));
    }

    #[test]
    fn a_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);

        assert!(m[0][0].approx_eq(-3.0, MARGIN));
        assert!(m[0][1].approx_eq(5.0, MARGIN));
        assert!(m[2][2].approx_eq(1.0, MARGIN));
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

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);

        assert_eq!(
            a.transpose(),
            Matrix::new(vec![
                vec![0.0, 9.0, 1.0, 0.0],
                vec![9.0, 8.0, 8.0, 0.0],
                vec![3.0, 0.0, 5.0, 5.0],
                vec![0.0, 8.0, 3.0, 8.0],
            ])
        )
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let a = Matrix::identity().transpose();

        assert_eq!(a, Matrix::identity());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);

        assert!(a.determinant().approx_eq(17.0, MARGIN));
    }

    #[test]
    fn submatrix_of_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);

        assert_eq!(
            a.submatrix(0, 2),
            Matrix::new(vec![vec![-3.0, 2.0], vec![0.0, 6.0],])
        )
    }

    #[test]
    fn submatrix_of_4x4_matrix() {
        let a = Matrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(
            a.submatrix(2, 1),
            Matrix::new(vec![
                vec![-6.0, 1.0, 6.0],
                vec![-8.0, 8.0, 6.0],
                vec![-7.0, -1.0, 1.0],
            ])
        )
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let b = a.submatrix(1, 0);

        assert!(b.determinant().approx_eq(25.0, MARGIN));
        assert!(a.minor(1, 0).approx_eq(25.0, MARGIN));
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert!(a.minor(0, 0).approx_eq(-12.0, MARGIN));
        assert!(a.cofactor(0, 0).approx_eq(-12.0, MARGIN));
        assert!(a.minor(1, 0).approx_eq(25.0, MARGIN));
        assert!(a.cofactor(1, 0).approx_eq(-25.0, MARGIN));
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);

        assert!(a.cofactor(0, 0).approx_eq(56.0, MARGIN));
        assert!(a.cofactor(0, 1).approx_eq(12.0, MARGIN));
        assert!(a.cofactor(0, 2).approx_eq(-46.0, MARGIN));
        assert!(a.determinant().approx_eq(-196.0, MARGIN));
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let a = Matrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);

        f_assert_eq!(a.cofactor(0, 0), 690.0);
        f_assert_eq!(a.cofactor(0, 1), 447.0);
        f_assert_eq!(a.cofactor(0, 2), 210.0);
        f_assert_eq!(a.cofactor(0, 3), 51.0);
        f_assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn test_invertible_matrix_is_invertible() {
        let a = Matrix::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);

        f_assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn test_noninvertible_matrix_is_not_invertible() {
        let a = Matrix::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        f_assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse_of_a_matrix() {
        let a = Matrix::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse().unwrap();

        f_assert_eq!(a.determinant(), 532.0);
        f_assert_eq!(a.cofactor(2, 3), -160.0);
        f_assert_eq!(b[3][2], -160.0 / 532.0);
        f_assert_eq!(a.cofactor(3, 2), 105.0);
        f_assert_eq!(b[2][3], 105.0 / 532.0);
        assert!(b.approx_eq(
            &Matrix::new(vec![
                vec![0.21805, 0.45113, 0.24060, -0.04511],
                vec![-0.80827, -1.45677, -0.44361, 0.52068],
                vec![-0.07895, -0.22368, -0.05263, 0.19737],
                vec![-0.52256, -0.81391, -0.30075, 0.30639],
            ]),
            MARGIN
        ));
    }

    #[test]
    fn inverse_of_another_matrix() {
        let a = Matrix::new(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);

        assert!(a.inverse().unwrap().approx_eq(
            &Matrix::new(vec![
                vec![-0.15385, -0.15385, -0.28205, -0.53846,],
                vec![-0.07692, 0.12308, 0.02564, 0.03077,],
                vec![0.35897, 0.35897, 0.43590, 0.92308,],
                vec![-0.69231, -0.69231, -0.76923, -1.92308],
            ]),
            MARGIN
        ));
    }

    #[test]
    fn inverse_of_a_third_matrix() {
        let a = Matrix::new(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);
        assert!(a.inverse().unwrap().approx_eq(
            &Matrix::new(vec![
                vec![-0.04074, -0.07778, 0.14444, -0.22222,],
                vec![-0.07778, 0.03333, 0.36667, -0.33333,],
                vec![-0.02901, -0.14630, -0.10926, 0.12963,],
                vec![0.17778, 0.06667, -0.26667, 0.33333],
            ]),
            MARGIN
        ));
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::new(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a.clone() * b.clone();

        assert!((c * b.inverse().unwrap()).approx_eq(&a, MARGIN));
    }
}
