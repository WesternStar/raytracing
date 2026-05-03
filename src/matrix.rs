use std::{f64::EPSILON, ops::Mul};

use crate::tuple::Tuple;

#[derive(Copy, Clone)]
pub struct Matrix<const WIDTH: usize, const HEIGHT: usize> {
    pub data: [[f64; WIDTH]; HEIGHT],
}
impl<const W: usize, const H: usize> PartialEq for Matrix<W, H> {
    fn eq(&self, other: &Self) -> bool {
        for j in 0..H{
            for i in 0..W{
                if (self.data[j][i]- other.data[j][i]).abs()> EPSILON{
                    return false;
                }
            }
        }
        return true;
    }
}

impl Mul for Matrix<4,4>{
    type Output = Matrix<4,4>;
    fn mul(self, other: Self) -> Self::Output {
        let mut m=Matrix {data: [[0.0;4];4]};
        for i in 0..4{
            for j in 0..4{
                m.data[j][i]= self.data[j][0]*other.data[0][i]+
                 self.data[j][1]*other.data[1][i]+
                 self.data[j][2]*other.data[2][i]+
                 self.data[j][3]*other.data[3][i];
            }
        }
        m
    }
}
impl Mul<Tuple> for Matrix<4,4>{
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Self::Output {
        let row = |j: usize| {
            self.data[j][0] * other.x
            + self.data[j][1] * other.y
            + self.data[j][2] * other.z
            + self.data[j][3] * other.w
        };
        Tuple::new(row(0), row(1), row(2), row(3))
    }
}
fn transpose(m: Matrix<4, 4>) -> Matrix<4, 4> {
    let mut res=Matrix {data: [[0.0;4];4]};
    for j in 0..4{
        for i in 0..4{
            res.data[i][j]=m.data[j][i]

        }
    }
    res
}

trait Det {
    fn det(&self) -> f64;
}

impl Det for Matrix<2, 2> {
    fn det(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}
fn submatrix3( m: &Matrix<3,3>,row: usize,col:usize)->Matrix<2,2>{
    let mut res = Matrix{data: [[0.0;2];2]};
    let rows: Vec<usize> =(0..3).filter(|&r|r!=row).collect();
    let cols: Vec<usize> =(0..3).filter(|&c|c!=col).collect();
    for (j, &r) in rows.iter().enumerate() {
        for (i, &c) in cols.iter().enumerate() {
            res.data[j][i] = m.data[r][c];
        }
    }
    res

}
fn minor3(m:&Matrix<3,3>,row:usize,col:usize)->f64{
    let sub = submatrix3(m, row, col);
    sub.det()
}
fn cofactor3(m: &Matrix<3, 3>, row: usize, col: usize) -> f64 {
    let minor = minor3(m, row, col);
    if (row + col) % 2 == 1 { -minor } else { minor }
}
impl Det for Matrix<3, 3> {
    fn det(&self) -> f64 {
        (0..3).map(|col|self.data[0][col]*cofactor3(self,
            0,col)).sum()
    }
}
fn submatrix4(m: &Matrix<4, 4>, row: usize, col: usize) -> Matrix<3, 3> {
    let mut res = Matrix{data: [[0.0;3];3]};
    let rows: Vec<usize> =(0..4).filter(|&r|r!=row).collect();
    let cols: Vec<usize> =(0..4).filter(|&c|c!=col).collect();
    for (j, &r) in rows.iter().enumerate() {
        for (i, &c) in cols.iter().enumerate() {
            res.data[j][i] = m.data[r][c];
        }
    }
    res

}

fn minor4(m:&Matrix<4,4>,row:usize,col:usize)->f64{
    let sub = submatrix4(m, row, col);
    sub.det()
}
fn cofactor4(m: &Matrix<4,4>, row: usize, col: usize) -> f64 {
    let minor = minor4(m, row, col);
    if (row + col) % 2 == 1 { -minor } else { minor }
}


fn is_invertible(m: &Matrix<4, 4>) -> bool {
    m.det().abs()> EPSILON
}

impl Det for Matrix<4, 4> {
    fn det(&self) -> f64 {
        (0..4).map(|col|self.data[0][col]*cofactor4(self,
            0,col)).sum()
    }
}
fn inverse(m: &Matrix<4, 4>) -> Matrix<4, 4> {
    let mut m2 = Matrix { data: [[0.0; 4]; 4] };
    let d = m.det();
    assert!(d.abs()>EPSILON);
    for row in 0..4 {
        for col in 0..4 {
            let c = cofactor4(m, row, col);
            m2.data[col][row] = c / d;
        }
    }
    m2
}

#[cfg(test)]
mod tests {
    use super::*;

//  Scenario: Constructing and inspecting a 4x4 matrix
//    Given the following 4x4 matrix M:
//      |  1   |  2   |  3   |  4   |
//      |  5.5 |  6.5 |  7.5 |  8.5 |
//      |  9   | 10   | 11   | 12   |
//      | 13.5 | 14.5 | 15.5 | 16.5 |
//    Then M[0,0] = 1    And M[0,3] = 4
//      And M[1,0] = 5.5  And M[1,2] = 7.5
//      And M[2,2] = 11   And M[3,0] = 13.5  And M[3,2] = 15.5
//  Scenario: A 2x2 matrix ought to be representable
//    Given the following 2x2 matrix M:
//      | -3 |  5 |
//      |  1 | -2 |
//    Then M[0,0] = -3  And M[0,1] = 5  And M[1,0] = 1  And M[1,1] = -2
//  Scenario: Calculating the determinant of a 2x2 matrix
//    Given 2x2 matrix A: | 1 | 5 | / | -3 | 2 |
//    Then det(A) = 17
//  Scenario: A submatrix of a 3x3 matrix is a 2x2 matrix
//    Given 3x3 matrix A: | 1 | 5 | 0 | / | -3 | 2 | 7 | / | 0 | 6 | -3 |
//    Then submatrix(A, 0, 2) = | -3 | 2 | / | 0 | 6 |
//  Scenario: A submatrix of a 4x4 matrix is a 3x3 matrix
//    Given 4x4 matrix A
//    Then submatrix(A, 2, 1) is the expected 3x3 matrix
//  Scenario: Calculating a minor of a 3x3 matrix
//    Given 3x3 matrix A, and B ← submatrix(A, 1, 0)
//    Then determinant(B) = 25  And minor(A, 1, 0) = 25
//  Scenario: Calculating a cofactor of a 3x3 matrix
//    Given 3x3 matrix A
//    Then minor(A,0,0)=-12  cofactor(A,0,0)=-12  minor(A,1,0)=25  cofactor(A,1,0)=-25
//  Scenario: Calculating the determinant of a 3x3 matrix
//    Given 3x3 matrix A: | 1 | 2 | 6 | / | -5 | 8 | -4 | / | 2 | 6 | 4 |
//    Then cofactor(A,0,0)=56  cofactor(A,0,1)=12  cofactor(A,0,2)=-46  det(A)=-196
//  Scenario: Testing an invertible matrix for invertibility
//    Given 4x4 matrix A
//    Then determinant(A) = -2120  And A is invertible
//  Scenario: Calculating the inverse of another matrix
//    Given 4x4 matrix A, Then inverse(A) matches expected values to 5dp
    #[test]
    fn test_inverse_2() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [ 8.0, -5.0,  9.0,  2.0],
                [ 7.0,  5.0,  6.0,  1.0],
                [-6.0,  0.0,  9.0,  6.0],
                [-3.0,  0.0, -9.0, -4.0],
            ],
        };
        let expected: Matrix<4, 4> = Matrix {
            data: [
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692,  0.12308,  0.02564,  0.03077],
                [ 0.35897,  0.35897,  0.43590,  0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ],
        };
        let inv = inverse(&a);
        for i in 0..4 {
            for j in 0..4 {
                assert!((inv.data[i][j] - expected.data[i][j]).abs() < 1e-4,
                    "mismatch at [{i}][{j}]: got {}, expected {}", inv.data[i][j], expected.data[i][j]);
            }
        }
    }

//  Scenario: Calculating the inverse of a third matrix
//    Given 4x4 matrix A, Then inverse(A) matches expected values to 5dp
    #[test]
    fn test_inverse_3() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [ 9.0,  3.0,  0.0,  9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0,  9.0,  6.0,  4.0],
                [-7.0,  6.0,  6.0,  2.0],
            ],
        };
        let expected: Matrix<4, 4> = Matrix {
            data: [
                [-0.04074, -0.07778,  0.14444, -0.22222],
                [-0.07778,  0.03333,  0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926,  0.12963],
                [ 0.17778,  0.06667, -0.26667,  0.33333],
            ],
        };
        let inv = inverse(&a);
        for i in 0..4 {
            for j in 0..4 {
                assert!((inv.data[i][j] - expected.data[i][j]).abs() < 1e-4,
                    "mismatch at [{i}][{j}]: got {}, expected {}", inv.data[i][j], expected.data[i][j]);
            }
        }
    }

    #[test]
    fn test_invertible() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [ 6.0,  4.0,  4.0,  4.0],
                [ 5.0,  5.0,  7.0,  6.0],
                [ 4.0, -9.0,  3.0, -7.0],
                [ 9.0,  1.0,  7.0, -6.0],
            ],
        };
        assert_eq!(a.det(), -2120.0);
        assert!(is_invertible(&a));
    }

//  Scenario: Testing a noninvertible matrix for invertibility
//    Given 4x4 matrix A with det = 0
//    Then determinant(A) = 0  And A is not invertible
    #[test]
    fn test_not_invertible() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [-4.0,  2.0, -2.0, -3.0],
                [ 9.0,  6.0,  2.0,  6.0],
                [ 0.0, -5.0,  1.0, -5.0],
                [ 0.0,  0.0,  0.0,  0.0],
            ],
        };
        assert_eq!(a.det(), 0.0);
        assert!(!is_invertible(&a));
    }

    #[test]
    fn test_det_3x3() {
        let a: Matrix<3, 3> = Matrix {
            data: [
                [ 1.0,  2.0,  6.0],
                [-5.0,  8.0, -4.0],
                [ 2.0,  6.0,  4.0],
            ],
        };
        assert_eq!(cofactor3(&a, 0, 0),  56.0);
        assert_eq!(cofactor3(&a, 0, 1),  12.0);
        assert_eq!(cofactor3(&a, 0, 2), -46.0);
        assert_eq!(a.det(), -196.0);
    }

//  Scenario: Calculating the determinant of a 4x4 matrix
//    Given 4x4 matrix A: | -2 | -8 | 3 | 5 | / | -3 | 1 | 7 | 3 | / | 1 | 2 | -9 | 6 | / | -6 | 7 | 7 | -9 |
//    Then cofactor(A,0,0)=690  cofactor(A,0,1)=447  cofactor(A,0,2)=210  cofactor(A,0,3)=51  det(A)=-4071
    #[test]
    fn test_det_4x4() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [-2.0, -8.0,  3.0,  5.0],
                [-3.0,  1.0,  7.0,  3.0],
                [ 1.0,  2.0, -9.0,  6.0],
                [-6.0,  7.0,  7.0, -9.0],
            ],
        };
        assert_eq!(cofactor4(&a, 0, 0),  690.0);
        assert_eq!(cofactor4(&a, 0, 1),  447.0);
        assert_eq!(cofactor4(&a, 0, 2),  210.0);
        assert_eq!(cofactor4(&a, 0, 3),   51.0);
        assert_eq!(a.det(), -4071.0);
    }

    #[test]
    fn test_cofactor3() {
        let a: Matrix<3, 3> = Matrix {
            data: [
                [ 3.0,  5.0,  0.0],
                [ 2.0, -1.0, -7.0],
                [ 6.0, -1.0,  5.0],
            ],
        };
        assert_eq!(minor3(&a, 0, 0),   -12.0);
        assert_eq!(cofactor3(&a, 0, 0), -12.0);
        assert_eq!(minor3(&a, 1, 0),    25.0);
        assert_eq!(cofactor3(&a, 1, 0), -25.0);
    }

    #[test]
    fn test_minor3() {
        let a: Matrix<3, 3> = Matrix {
            data: [
                [ 3.0,  5.0,  0.0],
                [ 2.0, -1.0, -7.0],
                [ 6.0, -1.0,  5.0],
            ],
        };
        let b = submatrix3(&a, 1, 0);
        assert_eq!(b.det(), 25.0);
        assert_eq!(minor3(&a, 1, 0), 25.0);
    }

    #[test]
    fn test_submatrix4() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [-6.0,  1.0,  1.0, 6.0],
                [-8.0,  5.0,  8.0, 6.0],
                [-1.0,  0.0,  8.0, 2.0],
                [-7.0,  1.0, -1.0, 1.0],
            ],
        };
        let expected: Matrix<3, 3> = Matrix {
            data: [
                [-6.0,  1.0, 6.0],
                [-8.0,  8.0, 6.0],
                [-7.0, -1.0, 1.0],
            ],
        };
        assert_eq!(submatrix4(&a, 2, 1).data, expected.data);
    }

    #[test]
    fn test_submatrix3() {
        let a: Matrix<3, 3> = Matrix {
            data: [
                [ 1.0, 5.0,  0.0],
                [-3.0, 2.0,  7.0],
                [ 0.0, 6.0, -3.0],
            ],
        };
        let expected: Matrix<2, 2> = Matrix {
            data: [
                [-3.0, 2.0],
                [ 0.0, 6.0],
            ],
        };
        assert_eq!(submatrix3(&a, 0, 2).data, expected.data);
    }

    #[test]
    fn test_det_2x2() {
        let a: Matrix<2, 2> = Matrix {
            data: [
                [ 1.0, 5.0],
                [-3.0, 2.0],
            ],
        };
        assert_eq!(a.det(), 17.0);
    }

    #[test]
    fn test_2x2_matrix() {
        let m: Matrix<2, 2> = Matrix {
            data: [
                [-3.0,  5.0],
                [ 1.0, -2.0],
            ],
        };
        assert_eq!(m.data[0][0], -3.0);
        assert_eq!(m.data[0][1],  5.0);
        assert_eq!(m.data[1][0],  1.0);
        assert_eq!(m.data[1][1], -2.0);
    }

//  Scenario: A 3x3 matrix ought to be representable
//    Given the following 3x3 matrix M:
//      | -3 |  5 |  0 |
//      |  1 | -2 | -7 |
//      |  0 |  1 |  1 |
//    Then M[0,0] = -3  And M[1,1] = -2  And M[2,2] = 1
    #[test]
    fn test_3x3_matrix() {
        let m: Matrix<3, 3> = Matrix {
            data: [
                [-3.0,  5.0,  0.0],
                [ 1.0, -2.0, -7.0],
                [ 0.0,  1.0,  1.0],
            ],
        };
        assert_eq!(m.data[0][0], -3.0);
        assert_eq!(m.data[1][1], -2.0);
        assert_eq!(m.data[2][2],  1.0);
    }

//  Scenario: Matrix equality with identical matrices
//    Given 4x4 matrices A and B with the same values
//    Then A = B
    #[test]
    fn test_matrix_equality() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let b: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        assert_eq!(a.data, b.data);
    }

//  Scenario: Matrix equality with different matrices
//    Given 4x4 matrices A and B with different values
//    Then A != B
    #[test]
    fn test_matrix_inequality() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let b: Matrix<4, 4> = Matrix {
            data: [
                [2.0, 3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0, 9.0],
                [8.0, 7.0, 6.0, 5.0],
                [4.0, 3.0, 2.0, 1.0],
            ],
        };
        assert_ne!(a.data, b.data);
    }

//  Scenario: Multiplying two matrices
//    Given A and B are 4x4 matrices
//    Then A * B equals the expected 4x4 matrix
    #[test]
    fn test_matrix_multiply() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let b: Matrix<4, 4> = Matrix {
            data: [
                [-2.0, 1.0, 2.0,  3.0],
                [ 3.0, 2.0, 1.0, -1.0],
                [ 4.0, 3.0, 6.0,  5.0],
                [ 1.0, 2.0, 7.0,  8.0],
            ],
        };
        let expected: Matrix<4, 4> = Matrix {
            data: [
                [20.0,  22.0,  50.0,  48.0],
                [44.0,  54.0, 114.0, 108.0],
                [40.0,  58.0, 110.0, 102.0],
                [16.0,  26.0,  46.0,  42.0],
            ],
        };
        assert_eq!((a * b).data, expected.data);
    }

//  Scenario: A matrix multiplied by a tuple
//    Given 4x4 matrix A and b ← tuple(1, 2, 3, 1)
//    Then A * b = tuple(18, 24, 33, 1)
    #[test]
    fn test_matrix_tuple_multiply() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let b = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
        let result = a * b;
        assert_eq!(result, Tuple { x: 18.0, y: 24.0, z: 33.0, w: 1.0 });
    }

//  Scenario: Multiplying a matrix by the identity matrix
//    Given 4x4 matrix A
//    Then A * identity_matrix = A
    #[test]
    fn test_matrix_identity_multiply() {
        let identity: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let a: Matrix<4, 4> = Matrix {
            data: [
                [0.0, 1.0,  2.0,  4.0],
                [1.0, 2.0,  4.0,  8.0],
                [2.0, 4.0,  8.0, 16.0],
                [4.0, 8.0, 16.0, 32.0],
            ],
        };
        assert_eq!((a * identity).data, a.data);
    }

//  Scenario: Multiplying the identity matrix by a tuple
//    Given a ← tuple(1, 2, 3, 4)
//    Then identity_matrix * a = a
    #[test]
    fn test_identity_tuple_multiply() {
        let identity: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let a = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        assert_eq!(identity * a, a);
    }

//  Scenario: Transposing a matrix
//    Given 4x4 matrix A
//    Then transpose(A) flips rows and columns
    #[test]
    fn test_transpose() {
        let a: Matrix<4, 4> = Matrix {
            data: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };
        let expected: Matrix<4, 4> = Matrix {
            data: [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ],
        };
        assert_eq!(transpose(a).data, expected.data);
    }

//  Scenario: Transposing the identity matrix
//    Given A ← transpose(identity_matrix)
//    Then A = identity_matrix
    #[test]
    fn test_transpose_identity() {
        let identity: Matrix<4, 4> = Matrix {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert_eq!(transpose(identity).data, identity.data);
    }

    #[test]
    fn test_4x4_matrix() {
        let m: Matrix<4, 4> = Matrix {
            data: [
                [ 1.0,  2.0,  3.0,  4.0],
                [ 5.5,  6.5,  7.5,  8.5],
                [ 9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };
        assert_eq!(m.data[0][0],  1.0);
        assert_eq!(m.data[0][3],  4.0);
        assert_eq!(m.data[1][0],  5.5);
        assert_eq!(m.data[1][2],  7.5);
        assert_eq!(m.data[2][2], 11.0);
        assert_eq!(m.data[3][0], 13.5);
        assert_eq!(m.data[3][2], 15.5);
    }
}
