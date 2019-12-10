/*
MIT License

Copyright (c) 2019 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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

    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn is_null(self) -> bool {
        self == Vec3::zero()
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

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        // https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector
        let n = normal.normalize();
        *self - 2.0 * (self.dot_product(n)) * n
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

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        use std::f64::EPSILON;;
        self.x <= other.x + EPSILON
            && self.x >= other.x - EPSILON
            && self.y <= other.y + EPSILON
            && self.y >= other.y - EPSILON
            && self.z <= other.z + EPSILON
            && self.z >= other.z - EPSILON
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

#[derive(Debug, Copy, Clone, Default)]
pub struct Mat3([[f64; 3]; 3]);

impl Mat3 {
    pub fn new() -> Self {
        Self::zero()
    }

    #[rustfmt::skip]
    pub fn id() -> Self {
        Mat3([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ])
    }

    #[rustfmt::skip]
    pub fn zero() -> Self {
        Mat3([[0.0; 3]; 3])
    }

    pub fn is_null(self) -> bool {
        self == Mat3::zero()
    }

    pub fn transformation_between(from: Vec3, to: Vec3) -> Self {
        // https://math.stackexchange.com/questions/180418/calculate-rotation-matrix-to-align-vector-a-to-vector-b-in-3d
        // https://gist.github.com/peteristhegreat/3b76d5169d7b9fc1e333
        let v = from.cross_product(to);
        if v.is_null() {
            return Mat3::id();
        }
        let ssc = Mat3([[0.0, -v.z, v.y], [v.z, 0.0, -v.x], [-v.y, v.x, 0.0]]);
        Mat3::id() + ssc + ((1.0 - from.dot_product(to)) / (v.norm().powi(2))) * ssc * ssc
    }
}

impl std::cmp::PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        use std::f64::EPSILON;;
        self.0
            .iter()
            .flatten()
            .zip(other.0.iter().flatten())
            .all(|(&left, &right)| left <= right + EPSILON && left >= right - EPSILON)
    }
}

impl std::ops::Add for Mat3 {
    type Output = Mat3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Mat3::new();
        let operands_iter = self.0.iter().flatten().zip(rhs.0.iter().flatten());
        result
            .0
            .iter_mut()
            .flatten()
            .zip(operands_iter)
            .for_each(|(res, (&left, &right))| *res = left + right);
        result
    }
}

impl std::ops::Mul<Mat3> for f64 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut result = Mat3::new();
        result
            .0
            .iter_mut()
            .flatten()
            .zip(rhs.0.iter().flatten())
            .for_each(|(res, &orig)| *res = self * orig);
        result
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

        #[test]
        fn same_direction_transform_is_id() {
            let vec = Vec3::new(1.1, 2.2, 2.2);
            assert_eq!(Mat3::id(), Mat3::transformation_between(vec, vec) );
        }
    }
}
