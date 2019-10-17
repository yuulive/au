//! # Units of measurement
//!
//! List of strongly typed units of measurement. It avoids the use of primitive
//! types.

use std::{
    convert::From,
    fmt::{Display, Formatter, LowerExp, UpperExp},
};

use num_traits::{Float, FloatConst};

/// Macro to implement Display trait for units. It passes the formatter options
/// to the unit inner type.
///
/// # Examples
/// ```
/// impl_display!(Seconds);
/// ```
macro_rules! impl_display {
    ($name:ident) => {
        /// Format the unit as its inner type.
        impl<T: Display + Float> Display for $name<T> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        /// Format the unit as its inner type.
        impl<T: LowerExp + Float> LowerExp for $name<T> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                LowerExp::fmt(&self.0, f)
            }
        }

        /// Format the unit as its inner type.
        impl<T: UpperExp + Float> UpperExp for $name<T> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                UpperExp::fmt(&self.0, f)
            }
        }
    };
}

/// Trait for the conversion to decibels.
pub trait Decibel<T> {
    /// Convert to decibels
    fn to_db(&self) -> T;
}

/// Implementation of the Decibels for f64
impl Decibel<f64> for f64 {
    /// Convert f64 to decibels
    fn to_db(&self) -> Self {
        20. * self.log10()
    }
}

/// Implementation of the Decibels for f32
impl Decibel<f32> for f32 {
    /// Convert f64 to decibels
    fn to_db(&self) -> Self {
        20. * self.log10()
    }
}

/// Unit of measurement: seconds [s]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Seconds<T: Float>(pub T);

/// Unit of measurement: Hertz [Hz]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Hertz<T: Float>(pub T);

/// Unit of measurement: Radiants per seconds [rad/s]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RadiantsPerSecond<T: Float>(pub T);

impl_display!(Seconds);
impl_display!(Hertz);
impl_display!(RadiantsPerSecond);

impl<T: Float + FloatConst> From<Hertz<T>> for RadiantsPerSecond<T> {
    fn from(hz: Hertz<T>) -> Self {
        Self((T::PI() + T::PI()) * hz.0)
    }
}

impl<T: Float + FloatConst> From<RadiantsPerSecond<T>> for Hertz<T> {
    fn from(rps: RadiantsPerSecond<T>) -> Self {
        Self(rps.0 / (T::PI() + T::PI()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ops::inv::Inv;

    #[test]
    fn decibel() {
        assert_abs_diff_eq!(40., 100_f64.to_db(), epsilon = 0.);
        assert_relative_eq!(-3.0103, 2_f64.inv().sqrt().to_db(), max_relative = 1e5);
    }

    #[test]
    fn conversion() {
        let tau = 2. * std::f64::consts::PI;
        assert_eq!(RadiantsPerSecond(tau), RadiantsPerSecond::from(Hertz(1.0)));

        let hz = Hertz(2.0);
        assert_eq!(hz, Hertz::from(RadiantsPerSecond::from(hz)));

        let rps = RadiantsPerSecond(2.0);
        assert_eq!(rps, RadiantsPerSecond::from(Hertz::from(rps)));
    }

    #[test]
    fn format() {
        assert_eq!("0.33".to_owned(), format!("{:.2}", Seconds(1. / 3.)));
        assert_eq!("0.3333".to_owned(), format!("{:.*}", 4, Seconds(1. / 3.)));
        assert_eq!("4.20e1".to_owned(), format!("{:.2e}", Seconds(42.)));
        assert_eq!("4.20E2".to_owned(), format!("{:.2E}", Seconds(420.)));
    }
}
