//! Rational functions
//!
//! Ratio whose numerator and denominator are polynomials.
//!
//! ```text
//!        b_n*x^n + b_(n-1)*x^(n-1) + ... + b_1*x + b_0
//! f(x) = ---------------------------------------------
//!        a_m*x^m + a_(m-1)*x^(m-1) + ... + a_1*x + a_0
//! ```

use nalgebra::RealField;
use num_complex::Complex;
use num_traits::{Float, One, Zero};

use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div, Mul},
};

use crate::polynomial::Poly;

mod arithmetic;

/// Rational function
#[derive(Clone, Debug, PartialEq)]
pub struct Rf<T> {
    /// Rational function numerator
    num: Poly<T>,
    /// Rational function denominator
    den: Poly<T>,
}

impl<T> Rf<T> {
    /// Create a new rational function given its numerator and denominator
    ///
    /// # Arguments
    ///
    /// * `num` - Rational function numerator
    /// * `den` - Rational function denominator
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// let rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
    /// ```
    #[must_use]
    pub fn new(num: Poly<T>, den: Poly<T>) -> Self {
        Self { num, den }
    }

    /// Extract rational function numerator
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// let num = poly!(1., 2.);
    /// let rf = Rf::new(num.clone(), poly!(-4., 6., -2.));
    /// assert_eq!(&num, rf.num());
    /// ```
    #[must_use]
    pub fn num(&self) -> &Poly<T> {
        &self.num
    }

    /// Extract rational function denominator
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// let den = poly!(-4., 6., -2.);
    /// let rf = Rf::new(poly!(1., 2.), den.clone());
    /// assert_eq!(&den, rf.den());
    /// ```
    #[must_use]
    pub fn den(&self) -> &Poly<T> {
        &self.den
    }
}

impl<T: Clone + PartialEq + Zero> Rf<T> {
    /// Calculate the relative degree between denominator and numerator.
    ///
    /// # Example
    /// ```
    /// use au::{num_traits::Inv, poly, Rf};
    /// let rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
    /// let expected = rf.relative_degree();
    /// assert_eq!(expected, 1);
    /// assert_eq!(rf.inv().relative_degree(), -1);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    pub fn relative_degree(&self) -> i32 {
        match (self.den.degree(), self.num.degree()) {
            (Some(d), Some(n)) => d as i32 - n as i32,
            (Some(d), None) => d as i32,
            (None, Some(n)) => -(n as i32),
            _ => 0,
        }
    }
}

impl<T: Float + RealField> Rf<T> {
    /// Calculate the poles of the rational function
    #[must_use]
    pub fn real_poles(&self) -> Option<Vec<T>> {
        self.den.real_roots()
    }

    /// Calculate the poles of the rational function
    #[must_use]
    pub fn complex_poles(&self) -> Vec<Complex<T>> {
        self.den.complex_roots()
    }

    /// Calculate the zeros of the rational function
    #[must_use]
    pub fn real_zeros(&self) -> Option<Vec<T>> {
        self.num.real_roots()
    }

    /// Calculate the zeros of the rational function
    #[must_use]
    pub fn complex_zeros(&self) -> Vec<Complex<T>> {
        self.num.complex_roots()
    }
}

impl<T: Clone + Div<Output = T> + One + PartialEq + Zero> Rf<T> {
    /// Normalization of rational function. If the denominator is zero the same
    /// rational function is returned.
    ///
    /// from:
    /// ```text
    ///        b_n*z^n + b_(n-1)*z^(n-1) + ... + b_1*z + b_0
    /// G(z) = ---------------------------------------------
    ///        a_n*z^n + a_(n-1)*z^(n-1) + ... + a_1*z + a_0
    /// ```
    /// to:
    /// ```text
    ///        b'_n*z^n + b'_(n-1)*z^(n-1) + ... + b'_1*z + b'_0
    /// G(z) = -------------------------------------------------
    ///          z^n + a'_(n-1)*z^(n-1) + ... + a'_1*z + a'_0
    /// ```
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// let rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
    /// let expected = Rf::new(poly!(-0.5, -1.), poly!(2., -3., 1.));
    /// assert_eq!(expected, rf.normalize());
    /// ```
    #[must_use]
    pub fn normalize(&self) -> Self {
        if self.den.is_zero() {
            return self.clone();
        }
        let (den, an) = self.den.monic();
        let num = &self.num / an;
        Self { num, den }
    }

    /// In place normalization of rational function. If the denominator is zero
    /// no operation is done.
    ///
    /// from:
    /// ```text
    ///        b_n*z^n + b_(n-1)*z^(n-1) + ... + b_1*z + b_0
    /// G(z) = ---------------------------------------------
    ///        a_n*z^n + a_(n-1)*z^(n-1) + ... + a_1*z + a_0
    /// ```
    /// to:
    /// ```text
    ///        b'_n*z^n + b'_(n-1)*z^(n-1) + ... + b'_1*z + b'_0
    /// G(z) = -------------------------------------------------
    ///          z^n + a'_(n-1)*z^(n-1) + ... + a'_1*z + a'_0
    /// ```
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// let mut rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
    /// rf.normalize_mut();
    /// let expected = Rf::new(poly!(-0.5, -1.), poly!(2., -3., 1.));
    /// assert_eq!(expected, rf);
    /// ```
    pub fn normalize_mut(&mut self) {
        if self.den.is_zero() {
            return;
        }
        let an = self.den.monic_mut();
        self.num.div_mut(&an);
    }
}

impl<T: Clone> Rf<T> {
    /// Evaluate the rational function.
    ///
    /// # Arguments
    ///
    /// * `s` - Value at which the rational function is evaluated.
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// use au::num_complex::Complex as C;
    /// let rf = Rf::new(poly!(1., 2., 3.), poly!(-4., -3., 1.));
    /// assert_eq!(-8.5, rf.eval_by_val(3.));
    /// assert_eq!(C::new(0.64, -0.98), rf.eval_by_val(C::new(0., 2.0_f32)));
    /// ```
    pub fn eval_by_val<N>(&self, s: N) -> N
    where
        N: Add<T, Output = N> + Clone + Div<Output = N> + Mul<Output = N> + Zero,
    {
        self.num.eval_by_val(s.clone()) / self.den.eval_by_val(s)
    }
}

impl<T> Rf<T> {
    /// Evaluate the rational function.
    ///
    /// # Arguments
    ///
    /// * `s` - Value at which the rational function is evaluated.
    ///
    /// # Example
    /// ```
    /// use au::{poly, Rf};
    /// use au::num_complex::Complex as C;
    /// let rf = Rf::new(poly!(1., 2., 3.), poly!(-4., -3., 1.));
    /// assert_eq!(-8.5, rf.eval(&3.));
    /// assert_eq!(C::new(0.64, -0.98), rf.eval(&C::new(0., 2.0_f32)));
    /// ```
    pub fn eval<'a, N>(&'a self, s: &'a N) -> N
    where
        T: 'a,
        N: 'a + Add<&'a T, Output = N> + Div<Output = N> + Mul<&'a N, Output = N> + Zero,
    {
        self.num.eval(s) / self.den.eval(s)
    }
}

/// Implementation of rational function printing
impl<T> Display for Rf<T>
where
    T: Display + One + PartialEq + PartialOrd + Zero,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (s_num, s_den) = if let Some(precision) = f.precision() {
            let num = format!("{poly:.prec$}", poly = self.num, prec = precision);
            let den = format!("{poly:.prec$}", poly = self.den, prec = precision);
            (num, den)
        } else {
            let num = format!("{}", self.num);
            let den = format!("{}", self.den);
            (num, den)
        };
        let length = s_num.len().max(s_den.len());
        let dash = "\u{2500}".repeat(length);
        write!(f, "{}\n{}\n{}", s_num, dash, s_den)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly;
    use num_complex::Complex;
    use num_traits::Inv;

    #[test]
    fn rational_function_creation() {
        let num = poly!(1., 2., 3.);
        let den = poly!(-4.2, -3.12, 0.0012);
        let rf = Rf::new(num.clone(), den.clone());
        assert_eq!(&num, rf.num());
        assert_eq!(&den, rf.den());
    }

    #[test]
    fn relative_degree() {
        let rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
        let expected = rf.relative_degree();
        assert_eq!(expected, 1);
        assert_eq!(rf.inv().relative_degree(), -1);
        assert_eq!(-1, Rf::new(poly!(1., 1.), Poly::zero()).relative_degree());
        assert_eq!(1, Rf::new(Poly::zero(), poly!(1., 1.)).relative_degree());
        assert_eq!(
            0,
            Rf::<f32>::new(Poly::zero(), Poly::zero()).relative_degree()
        );
    }

    #[test]
    fn evaluation() {
        let rf = Rf::new(poly!(-0.75, 0.25), poly!(0.75, 0.75, 1.));
        let res = rf.eval(&Complex::new(0., 0.9));
        assert_abs_diff_eq!(0.429, res.re, epsilon = 0.001);
        assert_abs_diff_eq!(1.073, res.im, epsilon = 0.001);
    }

    #[test]
    fn evaluation_by_value() {
        let rf = Rf::new(poly!(-0.75, 0.25), poly!(0.75, 0.75, 1.));
        let res1 = rf.eval(&Complex::new(0., 0.9));
        let res2 = rf.eval_by_val(Complex::new(0., 0.9));
        assert_eq!(res1, res2);
    }

    #[test]
    fn poles() {
        let rf = Rf::new(poly!(1.), poly!(6., -5., 1.));
        assert_eq!(Some(vec![2., 3.]), rf.real_poles());
    }

    #[test]
    fn complex_poles() {
        use num_complex::Complex32;
        let rf = Rf::new(poly!(1.), poly!(10., -6., 1.));
        assert_eq!(
            vec![Complex32::new(3., -1.), Complex32::new(3., 1.)],
            rf.complex_poles()
        );
    }

    #[test]
    fn zeros() {
        let rf = Rf::new(poly!(1.), poly!(6., -5., 1.));
        assert_eq!(None, rf.real_zeros());
    }

    #[test]
    fn complex_zeros() {
        use num_complex::Complex32;
        let rf = Rf::new(poly!(3.25, 3., 1.), poly!(10., -3., 1.));
        assert_eq!(
            vec![Complex32::new(-1.5, -1.), Complex32::new(-1.5, 1.)],
            rf.complex_zeros()
        );
    }

    #[test]
    fn print() {
        let rf = Rf::new(Poly::<f64>::one(), Poly::new_from_roots(&[-1.]));
        assert_eq!(
            "1\n\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\n1 +1s",
            format!("{}", rf)
        );

        let rf2 = Rf::new(poly!(1.123), poly!(0.987, -1.321));
        assert_eq!(
            "1.12\n\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\n0.99 -1.32s",
            format!("{:.2}", rf2)
        );
    }

    #[test]
    fn normalization() {
        let rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
        let expected = Rf::new(poly!(-0.5, -1.), poly!(2., -3., 1.));
        assert_eq!(expected, rf.normalize());

        let rf2 = Rf::new(poly!(1.), poly!(0.));
        assert_eq!(rf2, rf2.normalize());
    }

    #[test]
    fn normalization_mutable() {
        let mut rf = Rf::new(poly!(1., 2.), poly!(-4., 6., -2.));
        rf.normalize_mut();
        let expected = Rf::new(poly!(-0.5, -1.), poly!(2., -3., 1.));
        assert_eq!(expected, rf);

        let mut rf2 = Rf::new(poly!(1.), poly!(0.));
        let rf3 = rf2.clone();
        rf2.normalize_mut();
        assert_eq!(rf2, rf3);
    }

    #[test]
    fn eval_trasfer_function() {
        let s_num = Poly::new_from_coeffs(&[-1., 1.]);
        let s_den = Poly::new_from_coeffs(&[0., 1.]);
        let s = Rf::<f64>::new(s_num, s_den);
        let p = Poly::new_from_coeffs(&[1., 2., 3.]);
        let r = p.eval(&s);
        let expected = Rf::<f64>::new(poly!(3., -8., 6.), poly!(0., 0., 1.));
        assert_eq!(expected, r);
    }
}
