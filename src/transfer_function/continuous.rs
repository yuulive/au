//! Transfer functions for continuous time systems.

use nalgebra::{ComplexField, RealField, Scalar};
use num_complex::Complex;
use num_traits::{Float, FloatConst, MulAdd};

use std::marker::PhantomData;

use crate::{
    plots::{
        bode::{BodeIterator, BodePlot},
        polar::{PolarIterator, PolarPlot},
        root_locus::RootLocusIterator,
    },
    transfer_function::TfGen,
    units::{Decibel, RadiansPerSecond, Seconds},
    Continuous, Eval,
};

/// Continuous transfer function
pub type Tf<T> = TfGen<T, Continuous>;

impl<T: Float> Tf<T> {
    /// Time delay for continuous time transfer function.
    /// `y(t) = u(t - tau)`
    /// `G(s) = e^(-tau * s)
    ///
    /// # Arguments
    ///
    /// * `tau` - Time delay
    ///
    /// # Example
    /// ```
    /// use num_complex::Complex;
    /// use automatica::{units::Seconds, Tf};
    /// let d = Tf::delay(Seconds(2.));
    /// assert_eq!(1., d(Complex::new(0., 10.)).norm());
    /// ```
    pub fn delay(tau: Seconds<T>) -> impl Fn(Complex<T>) -> Complex<T> {
        move |s| (-s * tau.0).exp()
    }

    /// System inital value response to step input.
    /// `y(0) = G(s->infinity)`
    ///
    /// # Example
    /// ```
    /// use automatica::{poly, Tf};
    /// let tf = Tf::new(poly!(4.), poly!(1., 5.));
    /// assert_eq!(0., tf.init_value());
    /// ```
    pub fn init_value(&self) -> T {
        let n = self.num.degree();
        let d = self.den.degree();
        if n < d {
            T::zero()
        } else if n == d {
            self.num.leading_coeff() / self.den.leading_coeff()
        } else {
            T::infinity()
        }
    }

    /// System derivative inital value response to step input.
    /// `y'(0) = s * G(s->infinity)`
    ///
    /// # Example
    /// ```
    /// use automatica::{poly, Tf};
    /// let tf = Tf::new(poly!(1., -3.), poly!(1., 3., 2.));
    /// assert_eq!(-1.5, tf.init_value_der());
    /// ```
    pub fn init_value_der(&self) -> T {
        let n = self.num.degree();
        let d = self.den.degree().map(|d| d - 1);
        if n < d {
            T::zero()
        } else if n == d {
            self.num.leading_coeff() / self.den.leading_coeff()
        } else {
            T::infinity()
        }
    }

    /// Sensitivity function for the given controller `r`.
    /// ```text
    ///              1
    /// S(s) = -------------
    ///        1 + G(s)*R(s)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `r` - Controller
    ///
    /// # Example
    /// ```
    /// use automatica::{poly, Tf};
    /// let g = Tf::new(poly!(1.), poly!(0., 1.));
    /// let r = Tf::new(poly!(4.), poly!(1., 1.));
    /// let s = g.sensitivity(&r);
    /// assert_eq!(Tf::new(poly!(0., 1., 1.), poly!(4., 1., 1.)), s);
    /// ```
    pub fn sensitivity(&self, r: &Self) -> Self {
        let n = &self.num * &r.num;
        let d = &self.den * &r.den;
        Self {
            num: d.clone(),
            den: n + d,
            _type: PhantomData,
        }
    }

    /// Complementary sensitivity function for the given controller `r`.
    /// ```text
    ///          G(s)*R(s)
    /// F(s) = -------------
    ///        1 + G(s)*R(s)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `r` - Controller
    ///
    /// # Example
    /// ```
    /// use automatica::{poly, Tf};
    /// let g = Tf::new(poly!(1.), poly!(0., 1.));
    /// let r = Tf::new(poly!(4.), poly!(1., 1.));
    /// let f = g.compl_sensitivity(&r);
    /// assert_eq!(Tf::new(poly!(4.), poly!(4., 1., 1.)), f);
    /// ```
    pub fn compl_sensitivity(&self, r: &Self) -> Self {
        let l = self * r;
        l.feedback_n()
    }

    /// Sensitivity to control function for the given controller `r`.
    /// ```text
    ///            R(s)
    /// Q(s) = -------------
    ///        1 + G(s)*R(s)
    /// ```
    ///
    /// # Arguments
    ///
    /// * `r` - Controller
    ///
    /// # Example
    /// ```
    /// use automatica::{poly, Tf};
    /// let g = Tf::new(poly!(1.), poly!(0., 1.));
    /// let r = Tf::new(poly!(4.), poly!(1., 1.));
    /// let q = g.control_sensitivity(&r);
    /// assert_eq!(Tf::new(poly!(0., 4.), poly!(4., 1., 1.)), q);
    /// ```
    pub fn control_sensitivity(&self, r: &Self) -> Self {
        Self {
            num: &r.num * &self.den,
            den: &r.num * &self.num + &r.den * &self.den,
            _type: PhantomData,
        }
    }
}

impl<T: ComplexField + Float + RealField + Scalar> Tf<T> {
    /// Root locus for the given coefficient `k`
    ///
    /// # Arguments
    ///
    /// * `k` - Transfer function constant
    ///
    /// # Example
    /// ```
    /// use num_complex::Complex;
    /// use automatica::{poly, Poly, Tf};
    /// let l = Tf::new(poly!(1.), Poly::new_from_roots(&[-1., -2.]));
    /// let locus = l.root_locus(0.25);
    /// assert_eq!(Complex::new(-1.5, 0.), locus[0]);
    /// ```
    pub fn root_locus(&self, k: T) -> Vec<Complex<T>> {
        let p = &(&self.num * k) + &self.den;
        p.complex_roots()
    }

    /// Create a RootLocusIterator plot
    ///
    /// # Arguments
    ///
    /// * `min_k` - Minimum transfer constant of the plot
    /// * `max_k` - Maximum transfer constant of the plot
    /// * `step` - Step between each transfer constant
    ///
    /// `step` is linear.
    ///
    /// # Panics
    ///
    /// Panics if the step is not strictly positive of the minimum transfer constant
    /// is not lower than the maximum transfer constant.
    ///
    /// # Example
    /// ```
    /// use num_complex::Complex;
    /// use automatica::{poly, Poly, Tf};
    /// let l = Tf::new(poly!(1.), Poly::new_from_roots(&[-1., -2.]));
    /// let locus = l.root_locus_iter(0.1, 1.0, 0.05);
    /// assert_eq!(19, locus.count());
    /// ```
    pub fn root_locus_iter(self, min_k: T, max_k: T, step: T) -> RootLocusIterator<T> {
        RootLocusIterator::new(self, min_k, max_k, step)
    }
}

impl<T: Float + MulAdd<Output = T>> Tf<T> {
    /// Static gain `G(0)`.
    /// Ratio between constant output and constant input.
    /// Static gain is defined only for transfer functions of 0 type.
    ///
    /// Example
    ///
    /// ```
    /// use automatica::{poly, Tf};
    /// let tf = Tf::new(poly!(4., -3.),poly!(2., 5., -0.5));
    /// assert_eq!(2., tf.static_gain());
    /// ```
    pub fn static_gain(&self) -> T {
        self.eval(&T::zero())
    }
}

/// Implementation of the Bode plot for a transfer function
impl<T: Decibel<T> + Float + FloatConst + MulAdd<Output = T>> BodePlot<T> for Tf<T> {
    fn bode(
        self,
        min_freq: RadiansPerSecond<T>,
        max_freq: RadiansPerSecond<T>,
        step: T,
    ) -> BodeIterator<T> {
        BodeIterator::new(self, min_freq, max_freq, step)
    }
}

/// Implementation of the polar plot for a transfer function
impl<T: Float + FloatConst + MulAdd<Output = T>> PolarPlot<T> for Tf<T> {
    fn polar(
        self,
        min_freq: RadiansPerSecond<T>,
        max_freq: RadiansPerSecond<T>,
        step: T,
    ) -> PolarIterator<T> {
        PolarIterator::new(self, min_freq, max_freq, step)
    }
}

#[cfg(test)]
mod tests {
    use num_traits::One;

    use std::str::FromStr;

    use super::*;
    use crate::{poly, polynomial::Poly};

    #[test]
    fn delay() {
        let d = Tf::delay(Seconds(2.));
        assert_eq!(1., d(Complex::new(0., 10.)).norm());
        assert_eq!(-1., d(Complex::new(0., 0.5)).arg());
    }

    #[quickcheck]
    fn static_gain(g: f32) -> bool {
        let tf = Tf::new(poly!(g, -3.), poly!(1., 5., -0.5));
        g == tf.static_gain()
    }

    #[test]
    fn bode() {
        let tf = Tf::new(Poly::<f64>::one(), Poly::new_from_roots(&[-1.]));
        let b = tf.bode(RadiansPerSecond(0.1), RadiansPerSecond(100.0), 0.1);
        for g in b.into_db_deg() {
            assert!(g.magnitude() < 0.);
            assert!(g.phase() < 0.);
        }
    }

    #[test]
    fn polar() {
        let tf = Tf::new(poly!(5.), Poly::new_from_roots(&[-1., -10.]));
        let p = tf.polar(RadiansPerSecond(0.1), RadiansPerSecond(10.0), 0.1);
        for g in p {
            assert!(g.magnitude() < 1.);
            assert!(g.phase() < 0.);
        }
    }

    #[test]
    fn initial_value() {
        let tf = Tf::new(poly!(4.), poly!(1., 5.));
        assert_eq!(0., tf.init_value());
        let tf = Tf::new(poly!(4., -12.), poly!(1., 5.));
        assert_eq!(-2.4, tf.init_value());
        let tf = Tf::new(poly!(-3., 4.), poly!(5.));
        assert_eq!(std::f32::INFINITY, tf.init_value());
    }

    #[test]
    fn derivative_initial_value() {
        let tf = Tf::new(poly!(1., -3.), poly!(1., 3., 2.));
        assert_eq!(-1.5, tf.init_value_der());
        let tf = Tf::new(poly!(1.), poly!(1., 3., 2.));
        assert_eq!(0., tf.init_value_der());
        let tf = Tf::new(poly!(1., 0.5, -3.), poly!(1., 3., 2.));
        assert_eq!(std::f32::INFINITY, tf.init_value_der());
    }

    #[test]
    fn complementary_sensitivity() {
        let g = Tf::new(poly!(1.), poly!(0., 1.));
        let r = Tf::new(poly!(4.), poly!(1., 1.));
        let f = g.compl_sensitivity(&r);
        assert_eq!(Tf::new(poly!(4.), poly!(4., 1., 1.)), f);
    }

    #[test]
    fn sensitivity() {
        let g = Tf::new(poly!(1.), poly!(0., 1.));
        let r = Tf::new(poly!(4.), poly!(1., 1.));
        let s = g.sensitivity(&r);
        assert_eq!(Tf::new(poly!(0., 1., 1.), poly!(4., 1., 1.)), s);
    }

    #[test]
    fn control_sensitivity() {
        let g = Tf::new(poly!(1.), poly!(0., 1.));
        let r = Tf::new(poly!(4.), poly!(1., 1.));
        let q = g.control_sensitivity(&r);
        assert_eq!(Tf::new(poly!(0., 4.), poly!(4., 1., 1.)), q);
    }

    #[test]
    fn root_locus() {
        let l = Tf::new(poly!(1.), Poly::new_from_roots(&[-1., -2.]));

        let locus1 = l.root_locus(0.25);
        assert_eq!(Complex::from_str("-1.5").unwrap(), locus1[0]);

        let locus2 = l.root_locus(-2.);
        assert_eq!(Complex::from_str("0.").unwrap(), locus2[0]);
    }

    #[test]
    fn root_locus_iterations() {
        let l = Tf::new(poly!(1.0_f32), Poly::new_from_roots(&[0., -3., -5.]));
        let loci = l.root_locus_iter(1., 130., 1.);
        let last = loci.last().unwrap();
        dbg!(&last);
        assert_eq!(130., last.k());
        assert_eq!(3, last.output().len());
        assert!(last.output().iter().any(|r| r.re > 0.));
    }
}