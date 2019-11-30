#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn between_points(source: Vec3, destination: Vec3) -> Vec3 {
        destination - source
    }

    pub fn dot_product(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let norm = self.norm();
        Vec3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn distance(&self, other: Vec3) -> f64 {
        (other - *self).norm()
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Self> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mat3([[f64; 3]; 3]);

impl Mat3 {
    fn new() -> Self {
        Mat3([[0.0; 3]; 3])
    }

    #[rustfmt::skip]
    fn id() -> Self {
        Mat3([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ])
    }
}

impl std::ops::Add for Mat3 {
    type Output = Mat3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Mat3::new();
        let mut operands_iter = self.0.iter().flatten().zip(rhs.0.iter().flatten());
        let mut zipped_iter = result.0.iter_mut().flatten().zip(operands_iter);
        zipped_iter.for_each(|(res, (&left, &right))| *res = left + right);
        return result;
    }
}

impl std::ops::Mul<Mat3> for f64 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut result = Mat3::new();
        let mut zipped_iter = result.0.iter_mut().flatten().zip(rhs.0.iter().flatten());
        zipped_iter.for_each(|(res, &orig)| *res = self * orig);
        return result;
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mat = self.0;
        Vec3::new(
            rhs.x * mat[0][0] + rhs.y * mat[0][1] + rhs.z * mat[0][2],
            rhs.x * mat[1][0] + rhs.y * mat[1][1] + rhs.z * mat[1][2],
            rhs.x * mat[2][0] + rhs.y * mat[2][1] + rhs.z * mat[2][2],
        )
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let mat = self.0;
        let rhs = rhs.0;
        Mat3([
            [
                mat[0][0] * rhs[0][0] + mat[0][1] * rhs[1][0] + mat[0][2] * rhs[2][0],
                mat[0][0] * rhs[0][1] + mat[0][1] * rhs[1][1] + mat[0][2] * rhs[2][1],
                mat[0][0] * rhs[0][2] + mat[0][1] * rhs[1][2] + mat[0][2] * rhs[2][2],
            ],
            [
                mat[1][0] * rhs[0][0] + mat[1][1] * rhs[1][0] + mat[1][2] * rhs[2][0],
                mat[1][0] * rhs[0][1] + mat[1][1] * rhs[1][1] + mat[1][2] * rhs[2][1],
                mat[1][0] * rhs[0][2] + mat[1][1] * rhs[1][2] + mat[1][2] * rhs[2][2],
            ],
            [
                mat[2][0] * rhs[0][0] + mat[2][1] * rhs[1][0] + mat[2][2] * rhs[2][0],
                mat[2][0] * rhs[0][1] + mat[2][1] * rhs[1][1] + mat[2][2] * rhs[2][1],
                mat[2][0] * rhs[0][2] + mat[2][1] * rhs[1][2] + mat[2][2] * rhs[2][2],
            ],
        ])
    }
}

#[cfg(test)]
mod tests {
    #[rustfmt::skip]
    mod mat {
        use super::super::*;

        impl std::cmp::PartialEq for Mat3 {
            fn eq(&self, other: &Self) -> bool {
                use std::f64::EPSILON;;
                let mut zipped_iter = self.0.iter().flatten().zip(other.0.iter().flatten());
                zipped_iter.all(|(&left, &right)| left <= right + EPSILON && left >= right - EPSILON)
            }
        }

        impl std::cmp::PartialEq for Vec3 {
            fn eq(&self, other: &Self) -> bool {
                use std::f64::EPSILON;;
                self.x <= other.x + EPSILON && self.x >= other.x - EPSILON
                && self.y <= other.y + EPSILON && self.y >= other.y - EPSILON
                && self.z <= other.z + EPSILON && self.z >= other.z - EPSILON
            }
        }

        #[test]
        fn new_mat3_is_zeroed() {
            // Given a new matrix
            let mat = Mat3::new();
            // Then all elements are 0
            mat.0
                .iter()
                .flatten()
                .for_each(|&val| assert!(val.abs() < std::f64::EPSILON));
        }

        #[test]
        fn addition_example() {
            let mat_1 = Mat3([
                [0.0, 1.0, 2.0],
                [3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0]
            ]);

            let mat_2 = Mat3([
                [0.0, 1.0, -5.0],
                [3.0, 0.0, 5.0],
                [30.0, -2.0, 1.0]
            ]);
            let target = Mat3([
                [0.0, 2.0, -3.0],
                [6.0, 4.0, 10.0],
                [36.0, 5.0, 9.0]
            ]);
            let result = mat_1 + mat_2;
            assert_eq!(target, result);
        }

        #[test]
        fn multiply_with_constant_example() {
            let mat = Mat3([
                [0.0, 1.0, 2.0],
                [3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0]
            ]);
            let target =  Mat3([
                [0.0, 3.0, 6.0],
                [9.0, 12.0, 15.0],
                [18.0, 21.0, 24.0]
            ]);
            let result = 3.0 * mat;
            assert_eq!(target, result);
        }

        #[test]
        fn multiply_with_matrix_example() {
                let mat_1 = Mat3([
                [0.0, 1.0, 2.0],
                [-1.0, -2.0, 0.0],
                [10.0, 20.0, -100.0]
            ]);
                let mat_2 = Mat3([
                [0.0, 1.0, 2.0],
                [3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0]
            ]);
                let target =  Mat3([
                [15.0, 18.0, 21.0],
                [-6.0, -9.0, -12.0],
                [-540.0, -610.0, -680.0]
            ]);
            let result = mat_1 * mat_2;
            assert_eq!(target, result);
        }

        #[test]
        fn multiply_with_vec3_example() {
            let mat = Mat3([
                    [0.0, 1.0, 2.0],
                    [3.0, 4.0, 5.0],
                    [6.0, 7.0, 8.0]
                ]);

            let vec = Vec3::new(-10.0, 0.0, 22.0);
            let target = Vec3::new(44.0, 80.0, 116.0);
            let result = mat * vec;
            assert_eq!(target, result);
        }

        #[test]
        fn left_multiply_with_matrix_id_is_same() {
            let mat = Mat3([
                [0.0, 1.0, 2.0],
                [3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0]
            ]);
            let result = mat * Mat3::id();
            assert_eq!(mat, result);
        }

        #[test]
        fn right_multiply_with_matrix_id_is_same() {
            let mat = Mat3([
                [0.0, 1.0, 2.0],
                [3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0]
            ]);
            let result = Mat3::id() * mat;
            assert_eq!(mat, result);
        }

        #[test]
        fn mat3_id_with_vec3_is_vec3() {
            let vec = Vec3::new(-10.0, 0.0, 22.0);
            let result = Mat3::id() * vec;
            assert_eq!(vec, result);
        }
    }
}
