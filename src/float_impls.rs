use core::{num, intrinsics};
use core::f32::{NAN, NEG_INFINITY};

pub trait FloatImpls {
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_normal(self) -> bool;
    fn floor(self) -> f32;
    fn ceil(self) -> f32;
    fn round(self) -> f32;
    fn trunc(self) -> f32;
    fn fract(self) -> f32;
    fn signum(self) -> f32;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn mul_add(self, a: f32, b: f32) -> f32;
    fn powi(self, n: i32) -> f32;
    fn powf(self, n: f32) -> f32;
    fn sqrt(self) -> f32;
    fn exp(self) -> f32;
    fn exp2(self) -> f32;
    fn ln(self) -> f32;
    fn log(self, base: f32) -> f32;
    fn log2(self) -> f32;
    fn log10(self) -> f32;
    fn to_degrees(self) -> f32;
    fn max(self, other: f32) -> f32;
    fn min(self, other: f32) -> f32;
    fn asinh(self) -> f32;
    fn acosh(self) -> f32;
}

impl FloatImpls for f32 {
    /// Returns `true` if this value is `NaN` and false otherwise.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let nan = f32::NAN;
    /// let f = 7.0_f32;
    ///
    /// assert!(nan.is_nan());
    /// assert!(!f.is_nan());
    /// ```
    #[inline]
    fn is_nan(self) -> bool {
        num::Float::is_nan(self)
    }

    /// Returns `true` if this value is positive infinity or negative infinity and
    /// false otherwise.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let f = 7.0f32;
    /// let inf = f32::INFINITY;
    /// let neg_inf = f32::NEG_INFINITY;
    /// let nan = f32::NAN;
    ///
    /// assert!(!f.is_infinite());
    /// assert!(!nan.is_infinite());
    ///
    /// assert!(inf.is_infinite());
    /// assert!(neg_inf.is_infinite());
    /// ```
    #[inline]
    fn is_infinite(self) -> bool {
        num::Float::is_infinite(self)
    }

    /// Returns `true` if this number is neither infinite nor `NaN`.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let f = 7.0f32;
    /// let inf = f32::INFINITY;
    /// let neg_inf = f32::NEG_INFINITY;
    /// let nan = f32::NAN;
    ///
    /// assert!(f.is_finite());
    ///
    /// assert!(!nan.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// ```
    #[inline]
    fn is_finite(self) -> bool {
        num::Float::is_finite(self)
    }

    /// Returns `true` if the number is neither zero, infinite,
    /// [subnormal][subnormal], or `NaN`.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let min = f32::MIN_POSITIVE; // 1.17549435e-38f32
    /// let max = f32::MAX;
    /// let lower_than_min = 1.0e-40_f32;
    /// let zero = 0.0_f32;
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!f32::NAN.is_normal());
    /// assert!(!f32::INFINITY.is_normal());
    /// // Values between `0` and `min` are Subnormal.
    /// assert!(!lower_than_min.is_normal());
    /// ```
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    #[inline]
    fn is_normal(self) -> bool {
        num::Float::is_normal(self)
    }

    /// Returns the largest integer less than or equal to a number.
    ///
    /// ```
    /// let f = 3.99_f32;
    /// let g = 3.0_f32;
    ///
    /// assert_eq!(f.floor(), 3.0);
    /// assert_eq!(g.floor(), 3.0);
    /// ```
    #[inline]
    fn floor(self) -> f32 {
        use core::mem;
        let mut self_int: u32 = unsafe { mem::transmute(self) };
        let e = (((self_int >> 23) & 0xff).wrapping_sub(0x7f)) as i32;

        if e >= 23 {
            return self;
        }
        if e >= 0 {
            let m: u32 = 0x007fffff >> e;
            if (self_int & m) == 0 {
                return self;
            }
            if self_int >> 31 != 0 {
                self_int += m;
            }
            self_int &= !m;
        } else {
            if self_int >> 31 == 0 {
                self_int = 0;
            } else if self_int << 1 != 0 {
                self_int = unsafe {mem::transmute(-1.0f32)};
            }
        }
        unsafe { mem::transmute(self_int) }
    }

    /// Returns the smallest integer greater than or equal to a number.
    ///
    /// ```
    /// let f = 3.01_f32;
    /// let g = 4.0_f32;
    ///
    /// assert_eq!(f.ceil(), 4.0);
    /// assert_eq!(g.ceil(), 4.0);
    /// ```
    #[inline]
    fn ceil(self) -> f32 {
        -(-self).floor()
    }

    /// Returns the nearest integer to a number. Round half-way cases away from
    /// `0.0`.
    ///
    /// ```
    /// let f = 3.3_f32;
    /// let g = -3.3_f32;
    ///
    /// assert_eq!(f.round(), 3.0);
    /// assert_eq!(g.round(), -3.0);
    /// ```
    #[inline]
    fn round(self) -> f32 {
        unsafe { intrinsics::roundf32(self) }
    }

    /// Returns the integer part of a number.
    ///
    /// ```
    /// let f = 3.3_f32;
    /// let g = -3.7_f32;
    ///
    /// assert_eq!(f.trunc(), 3.0);
    /// assert_eq!(g.trunc(), -3.0);
    /// ```
    #[inline]
    fn trunc(self) -> f32 {
        unsafe { intrinsics::truncf32(self) }
    }

    /// Returns the fractional part of a number.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let x = 3.5_f32;
    /// let y = -3.5_f32;
    /// let abs_difference_x = (x.fract() - 0.5).abs();
    /// let abs_difference_y = (y.fract() - (-0.5)).abs();
    ///
    /// assert!(abs_difference_x <= f32::EPSILON);
    /// assert!(abs_difference_y <= f32::EPSILON);
    /// ```
    #[inline]
    fn fract(self) -> f32 {
        self - self.trunc()
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    ///
    /// ```
    /// use std::f32;
    ///
    /// let f = 3.5_f32;
    ///
    /// assert_eq!(f.signum(), 1.0);
    /// assert_eq!(f32::NEG_INFINITY.signum(), -1.0);
    ///
    /// assert!(f32::NAN.signum().is_nan());
    /// ```
    #[inline]
    fn signum(self) -> f32 {
        num::Float::signum(self)
    }

    /// Returns `true` if `self`'s sign bit is positive, including
    /// `+0.0` and `INFINITY`.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let nan = f32::NAN;
    /// let f = 7.0_f32;
    /// let g = -7.0_f32;
    ///
    /// assert!(f.is_sign_positive());
    /// assert!(!g.is_sign_positive());
    /// // Requires both tests to determine if is `NaN`
    /// assert!(!nan.is_sign_positive() && !nan.is_sign_negative());
    /// ```
    #[inline]
    fn is_sign_positive(self) -> bool {
        num::Float::is_sign_positive(self)
    }

    /// Returns `true` if `self`'s sign is negative, including `-0.0`
    /// and `NEG_INFINITY`.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let nan = f32::NAN;
    /// let f = 7.0f32;
    /// let g = -7.0f32;
    ///
    /// assert!(!f.is_sign_negative());
    /// assert!(g.is_sign_negative());
    /// // Requires both tests to determine if is `NaN`.
    /// assert!(!nan.is_sign_positive() && !nan.is_sign_negative());
    /// ```
    #[inline]
    fn is_sign_negative(self) -> bool {
        num::Float::is_sign_negative(self)
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error. This produces a more accurate result with better performance than
    /// a separate multiplication operation followed by an add.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let m = 10.0_f32;
    /// let x = 4.0_f32;
    /// let b = 60.0_f32;
    ///
    /// // 100.0
    /// let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn mul_add(self, a: f32, b: f32) -> f32 {
        unsafe { intrinsics::fmaf32(self, a, b) }
    }

    /// Raises a number to an integer power.
    ///
    /// Using this function is generally faster than using `powf`
    ///
    /// ```
    /// use std::f32;
    ///
    /// let x = 2.0_f32;
    /// let abs_difference = (x.powi(2) - x*x).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn powi(self, n: i32) -> f32 {
        num::Float::powi(self, n)
    }

    /// Raises a number to a floating point power.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let x = 2.0_f32;
    /// let abs_difference = (x.powf(2.0) - x*x).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn powf(self, n: f32) -> f32 {
        // see notes above in `floor`
        #[cfg(target_env = "msvc")]
        return (self as f64).powf(n as f64) as f32;
        #[cfg(not(target_env = "msvc"))]
        return unsafe { intrinsics::powf32(self, n) };
    }

    /// Takes the square root of a number.
    ///
    /// Returns NaN if `self` is a negative number.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let positive = 4.0_f32;
    /// let negative = -4.0_f32;
    ///
    /// let abs_difference = (positive.sqrt() - 2.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// assert!(negative.sqrt().is_nan());
    /// ```
    #[inline]
    fn sqrt(self) -> f32 {
        if self < 0.0 {
            NAN
        } else {
            unsafe { intrinsics::sqrtf32(self) }
        }
    }

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// ```
    /// use std::f32;
    ///
    /// let one = 1.0f32;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn exp(self) -> f32 {
        // see notes above in `floor`
        #[cfg(target_env = "msvc")]
        return (self as f64).exp() as f32;
        #[cfg(not(target_env = "msvc"))]
        return unsafe { intrinsics::expf32(self) };
    }

    /// Returns `2^(self)`.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let f = 2.0f32;
    ///
    /// // 2^2 - 4 == 0
    /// let abs_difference = (f.exp2() - 4.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn exp2(self) -> f32 {
        unsafe { intrinsics::exp2f32(self) }
    }

    /// Returns the natural logarithm of the number.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let one = 1.0f32;
    /// // e^1
    /// let e = one.exp();
    ///
    /// // ln(e) - 1 == 0
    /// let abs_difference = (e.ln() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn ln(self) -> f32 {
        // see notes above in `floor`
        #[cfg(target_env = "msvc")]
        return (self as f64).ln() as f32;
        #[cfg(not(target_env = "msvc"))]
        return unsafe { intrinsics::logf32(self) };
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let ten = 10.0f32;
    /// let two = 2.0f32;
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference_10 = (ten.log(10.0) - 1.0).abs();
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference_2 = (two.log(2.0) - 1.0).abs();
    ///
    /// assert!(abs_difference_10 <= f32::EPSILON);
    /// assert!(abs_difference_2 <= f32::EPSILON);
    /// ```
    #[inline]
    fn log(self, base: f32) -> f32 {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let two = 2.0f32;
    ///
    /// // log2(2) - 1 == 0
    /// let abs_difference = (two.log2() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn log2(self) -> f32 {
        #[cfg(target_os = "android")]
        return ::sys::android::log2f32(self);
        #[cfg(not(target_os = "android"))]
        return unsafe { intrinsics::log2f32(self) };
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let ten = 10.0f32;
    ///
    /// // log10(10) - 1 == 0
    /// let abs_difference = (ten.log10() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn log10(self) -> f32 {
        // see notes above in `floor`
        #[cfg(target_env = "msvc")]
        return (self as f64).log10() as f32;
        #[cfg(not(target_env = "msvc"))]
        return unsafe { intrinsics::log10f32(self) };
    }

    /// Converts radians to degrees.
    ///
    /// ```
    /// use std::f32::{self, consts};
    ///
    /// let angle = consts::PI;
    ///
    /// let abs_difference = (angle.to_degrees() - 180.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn to_degrees(self) -> f32 {
        num::Float::to_degrees(self)
    }

    /// Returns the maximum of the two numbers.
    ///
    /// ```
    /// let x = 1.0f32;
    /// let y = 2.0f32;
    ///
    /// assert_eq!(x.max(y), y);
    /// ```
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    #[inline]
    fn max(self, other: f32) -> f32 {
        if self > other { self } else { other }
    }

    /// Returns the minimum of the two numbers.
    ///
    /// ```
    /// let x = 1.0f32;
    /// let y = 2.0f32;
    ///
    /// assert_eq!(x.min(y), x);
    /// ```
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    #[inline]
    fn min(self, other: f32) -> f32 {
        if self < other { self } else { other }
    }

    /// Inverse hyperbolic sine function.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let x = 1.0f32;
    /// let f = x.sinh().asinh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn asinh(self) -> f32 {
        if self == NEG_INFINITY {
            NEG_INFINITY
        } else {
            (self + ((self * self) + 1.0).sqrt()).ln()
        }
    }

    /// Inverse hyperbolic cosine function.
    ///
    /// ```
    /// use std::f32;
    ///
    /// let x = 1.0f32;
    /// let f = x.cosh().acosh();
    ///
    /// let abs_difference = (f - x).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    fn acosh(self) -> f32 {
        match self {
            x if x < 1.0 => NAN,
            x => (x + ((x * x) - 1.0).sqrt()).ln(),
        }
    }
}
