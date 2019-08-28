//! # Bode plot
//!
//! Bode plot returns the angular frequency, the magnitude and the phase.
//!
//! Functions use angular frequencies as default inputs and output, being the
//! inverse of the poles and zeros time constants.

use crate::{transfer_function::Tf, Decibel, Eval};
use num_complex::Complex64;

/// Struct for the calculation of Bode plots
#[derive(Debug)]
pub struct BodeIterator {
    /// Transfer function
    tf: Tf,
    /// Number of intervals of the plot
    intervals: f64,
    /// Step between frequencies
    step: f64,
    /// Start frequency
    base_freq: f64,
    /// Current data index
    index: f64,
}

impl BodeIterator {
    /// Create a BodeIterator struct
    ///
    /// # Arguments
    ///
    /// * `tf` - Transfer function to plot
    /// * `min_freq` - Minimum angular frequency of the plot
    /// * `max_freq` - Maximum angular frequency of the plot
    /// * `step` - Step between frequencies
    ///
    /// `step` shall be in logarithmic scale. Use 0.1 to have 10 point per decade
    ///
    /// # Panics
    ///
    /// Panics if the step is not strictly positive of the minimum frequency
    /// is not lower than the maximum frequency
    pub(crate) fn new(tf: Tf, min_freq: f64, max_freq: f64, step: f64) -> Self {
        assert!(step > 0.0);
        assert!(min_freq < max_freq);

        let min = min_freq.log10();
        let max = max_freq.log10();
        let intervals = ((max - min) / step).floor();
        Self {
            tf,
            intervals,
            step,
            base_freq: min,
            index: 0.0,
        }
    }

    /// Convert BodeIterator into decibels and degrees
    pub fn into_db_deg(self) -> impl Iterator<Item = Bode> {
        self.map(|g| Bode {
            magnitude: g.magnitude.to_db(),
            phase: g.phase.to_degrees(),
            ..g
        })
    }
}

/// Struct to hold the data returned by the Bode iterator
pub struct Bode {
    /// Angular frequency (rad)
    angular_frequency: f64,
    /// Magnitude (absolute value or dB)
    magnitude: f64,
    /// Phase (rad or degrees)
    phase: f64,
}

/// Implementation of Bode methods
impl Bode {
    /// Get the angular frequency
    pub fn angular_frequency(&self) -> f64 {
        self.angular_frequency
    }

    /// Get the frequency
    pub fn frequency(&self) -> f64 {
        self.angular_frequency / 2. / std::f64::consts::PI
    }

    /// Get the magnitude
    pub fn magnitude(&self) -> f64 {
        self.magnitude
    }

    /// Get the phase
    pub fn phase(&self) -> f64 {
        self.phase
    }
}

/// Implementation of the Iterator trait for `BodeIterator` struct
impl Iterator for BodeIterator {
    type Item = Bode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.intervals {
            None
        } else {
            let freq_exponent = self.step.mul_add(self.index, self.base_freq);
            let omega = 10f64.powf(freq_exponent);
            let jomega = Complex64::new(0.0, omega);
            let g = self.tf.eval(&jomega);
            self.index += 1.;
            Some(Bode {
                angular_frequency: omega,
                magnitude: g.norm(),
                phase: g.arg(),
            })
        }
    }
}

/// Trait for the implementation of Bode plot for a linear system.
pub trait BodePlot {
    /// Create a BodeIterator struct
    ///
    /// # Arguments
    ///
    /// * `min_freq` - Minimum angular frequency of the plot
    /// * `max_freq` - Maximum angular frequency of the plot
    /// * `step` - Step between frequencies
    ///
    /// `step` shall be in logarithmic scale. Use 0.1 to have 10 point per decade
    ///
    /// # Panics
    ///
    /// Panics if the step is not strictly positive of the minimum frequency
    /// is not lower than the maximum frequency
    fn bode(self, min_freq: f64, max_freq: f64, step: f64) -> BodeIterator;

    /// Create a BodeIterator struct
    ///
    /// # Arguments
    ///
    /// * `min_freq` - Minimum frequency of the plot
    /// * `max_freq` - Maximum frequency of the plot
    /// * `step` - Step between frequencies
    ///
    /// `step` shall be in logarithmic scale. Use 0.1 to have 10 point per decade
    ///
    /// # Panics
    ///
    /// Panics if the step is not strictly positive of the minimum frequency
    /// is not lower than the maximum frequency
    fn bode_hz(self, min_freq: f64, max_freq: f64, step: f64) -> BodeIterator
    where
        Self: std::marker::Sized,
    {
        let tau = 2. * std::f64::consts::PI;
        self.bode(tau * min_freq, tau * max_freq, step)
    }
}
