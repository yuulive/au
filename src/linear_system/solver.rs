use crate::linear_system::Ss;

use nalgebra::DVector;

/// Struct for the time evolution of a linear system
#[derive(Debug)]
pub struct Rk2Iterator<'a> {
    /// Linear system
    sys: &'a Ss,
    /// Input vector,
    input: DVector<f64>,
    /// State vector.
    state: DVector<f64>,
    /// Output vector.
    output: DVector<f64>,
    /// Interval.
    h: f64,
    /// Number of steps.
    n: usize,
    /// Index.
    index: usize,
}

impl<'a> Rk2Iterator<'a> {
    /// Response to step function, using Runge-Kutta second order method
    ///
    /// # Arguments
    ///
    /// * `u` - input vector (colum mayor)
    /// * `x0` - initial state (colum mayor)
    /// * `h` - integration time interval
    /// * `n` - integration steps
    pub(crate) fn new(sys: &'a Ss, u: &[f64], x0: &[f64], h: f64, n: usize) -> Self {
        let input = DVector::from_column_slice(u);
        let state = DVector::from_column_slice(x0);
        // Calculate the output at time 0.
        let output = &sys.c * &state + &sys.d * &input;
        Self {
            sys,
            input,
            state,
            output,
            h,
            n,
            index: 0,
        }
    }

    /// Runge-Kutta order 2 method
    fn rk2(&mut self) -> Option<Rk2> {
        // y_n+1 = y_n + 1/2(k1 + k2) + O(h^3)
        // k1 = h*f(x_n, y_n)
        // k2 = h*f(x_n + h, y_n + k1)
        //
        // x_n (time) does not explicitly appear for a linear system with
        // input a step function
        let bu = &self.sys.b * &self.input;
        let k1 = self.h * (&self.sys.a * &self.state + &bu);
        let k2 = self.h * (&self.sys.a * (&self.state + &k1) + &bu);
        self.state += 0.5 * (k1 + k2);
        self.output = &self.sys.c * &self.state + &self.sys.d * &self.input;

        self.index += 1;
        Some(Rk2 {
            time: self.index as f64 * self.h,
            state: self.state.as_slice().to_vec(),
            output: self.output.as_slice().to_vec(),
        })
    }
}

/// Implementation of the Iterator trait for the Rk2Iterator struct
impl<'a> Iterator for Rk2Iterator<'a> {
    type Item = Rk2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.n {
            None
        } else if self.index == 0 {
            self.index += 1;
            // State and output at time 0.
            Some(Rk2 {
                time: 0.,
                state: self.state.as_slice().to_vec(),
                output: self.output.as_slice().to_vec(),
            })
        } else {
            self.rk2()
        }
    }
}

/// Struct to hold the data of the linear system time evolution
#[derive(Debug)]
pub struct Rk2 {
    /// Time of the current step
    time: f64,
    /// Current state
    state: Vec<f64>,
    /// Current output
    output: Vec<f64>,
}

impl Rk2 {
    /// Get the time of the current step
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Get the current state of the system
    pub fn state(&self) -> &Vec<f64> {
        &self.state
    }

    /// Get the current output of the system
    pub fn output(&self) -> &Vec<f64> {
        &self.output
    }
}

/// Struct for the time evolution of a linear system
#[derive(Debug)]
pub struct Rkf45Iterator<'a> {
    /// Linear system
    sys: &'a Ss,
    /// Input vector,
    input: DVector<f64>,
    /// State vector.
    state: DVector<f64>,
    /// Output vector.
    output: DVector<f64>,
    /// Interval.
    h: f64,
    /// Number of steps.
    n: usize,
    /// Index.
    index: usize,
}

impl<'a> Rkf45Iterator<'a> {
    /// Response to step function, using Runge-Kutta-Fehlberg method
    ///
    /// # Arguments
    ///
    /// * `u` - input vector (colum mayor)
    /// * `x0` - initial state (colum mayor)
    /// * `h` - integration time interval
    /// * `n` - integration steps
    pub(crate) fn new(sys: &'a Ss, u: &[f64], x0: &[f64], h: f64, n: usize) -> Self {
        let input = DVector::from_column_slice(u);
        let state = DVector::from_column_slice(x0);
        // Calculate the output at time 0.
        let output = &sys.c * &state + &sys.d * &input;
        Self {
            sys,
            input,
            state,
            output,
            h,
            n,
            index: 0,
        }
    }

    /// Runge-Kutta-Fehlberg order 4 and 5 method with adaptive step size
    fn rkf45(&mut self) -> Option<Rkf45> {
        let bu = &self.sys.b * &self.input;
        let tol = 1e-4;
        let mut error;
        loop {
            let k1 = self.h * (&self.sys.a * &self.state + &bu);
            let k2 = self.h * (&self.sys.a * (&self.state + B21 * &k1) + &bu);
            let k3 = self.h * (&self.sys.a * (&self.state + B3[0] * &k1 + B3[1] * &k2) + &bu);
            let k4 = self.h
                * (&self.sys.a * (&self.state + B4[0] * &k1 + B4[1] * &k2 + B4[2] * &k3) + &bu);
            let k5 = self.h
                * (&self.sys.a
                    * (&self.state + B5[0] * &k1 + B5[1] * &k2 + B5[2] * &k3 + B5[3] * &k4)
                    + &bu);
            let k6 = self.h
                * (&self.sys.a
                    * (&self.state
                        + B6[0] * &k1
                        + B6[1] * &k2
                        + B6[2] * &k3
                        + B6[3] * &k4
                        + B6[4] * &k5)
                    + &bu);

            let xn1 = &self.state + C[0] * &k1 + C[1] * &k3 + C[2] * &k4 + C[3] * &k5;
            let xn1_ = &self.state + D[0] * &k1 + D[1] * &k3 + D[2] * &k4 + D[3] * &k5 + D[4] * &k6;

            error = (&xn1 - &xn1_).abs().max();
            let error_ratio = tol / error;
            if error < tol {
                self.h = 0.95 * self.h * error_ratio.powf(0.25);
                self.state = xn1;
                break;
            }
            self.h = 0.95 * self.h * error_ratio.powf(0.2);
        }
        self.output = &self.sys.c * &self.state + &self.sys.d * &self.input;

        self.index += 1;
        Some(Rkf45 {
            time: self.h,
            state: self.state.as_slice().to_vec(),
            output: self.output.as_slice().to_vec(),
            error,
        })
    }
}

/// Implementation of the Iterator trait for the Rkf45Iterator struct
impl<'a> Iterator for Rkf45Iterator<'a> {
    type Item = Rkf45;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.n {
            None
        } else if self.index == 0 {
            self.index += 1;
            // Sater and output at time 0.
            Some(Rkf45 {
                time: 0.,
                state: self.state.as_slice().to_vec(),
                output: self.output.as_slice().to_vec(),
                error: 0.,
            })
        } else {
            self.rkf45()
        }
    }
}

// Coefficients of th Butcher table of rkf45 method.
const B21: f64 = 1. / 4.;
const B3: [f64; 2] = [3. / 32., 9. / 32.];
const B4: [f64; 3] = [1932. / 2197., -7200. / 2197., 7296. / 2197.];
const B5: [f64; 4] = [439. / 216., -8., 3680. / 513., -845. / 4104.];
const B6: [f64; 5] = [-8. / 27., 2., -3544. / 2565., 1859. / 4104., -11. / 40.];
const C: [f64; 4] = [25. / 216., 1408. / 2564., 2197. / 4101., -1. / 5.];
const D: [f64; 5] = [
    16. / 135.,
    6656. / 12_825.,
    28_561. / 56_430.,
    -9. / 50.,
    2. / 55.,
];
//////

/// Struct to hold the data of the linar system time evolution
#[derive(Debug)]
pub struct Rkf45 {
    /// Current step size
    time: f64,
    /// Current state
    state: Vec<f64>,
    /// Current output
    output: Vec<f64>,
    /// Current maximum absolute error
    error: f64,
}

impl Rkf45 {
    /// Get the time of the current step
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Get the current state of the system
    pub fn state(&self) -> &Vec<f64> {
        &self.state
    }

    /// Get the current output of the system
    pub fn output(&self) -> &Vec<f64> {
        &self.output
    }

    /// Get the current maximum absolute error
    pub fn error(&self) -> f64 {
        self.error
    }
}