/*
MIT License

Copyright (c) 2019, 2020 Vincent Hiribarren

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

use crate::UnitInterval;

pub fn unit_interval_clamp(val: f64) -> UnitInterval {
    match val {
        x if x < 0.0 => 0.0,
        x if x > 1.0 => 1.0,
        x => x,
    }
}

// Not using generics, because it would require a "float" trait which does not
// exist in the std library, and adding a 3rd party lib only for that is excessive.
#[inline]
pub(crate) fn f64_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= std::f64::EPSILON
}

#[inline]
pub(crate) fn f64_lt(a: f64, b: f64) -> bool {
    a <= b + std::f64::EPSILON
}

#[inline]
pub(crate) fn f64_gt(a: f64, b: f64) -> bool {
    a >= b - std::f64::EPSILON
}
