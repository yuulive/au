use num_complex::Complex;
use num_traits::{Float, FloatConst, NumCast};

use std::{
    fmt::Debug,
    ops::{Mul, Sub},
};

use super::*;

/// Structure to hold the computational data for polynomial root finding.
#[derive(Debug)]
pub(super) struct RootsFinder<T> {
    /// Polynomial
    poly: Poly<T>,
    /// Polynomial derivative
    der: Poly<T>,
    /// Solution, roots of the polynomial
    solution: Vec<Complex<T>>,
    /// Maximum iterations of the algorithm
    iterations: u32,
}

impl<T: Float + FloatConst + NumCast> RootsFinder<T> {
    /// Create a `RootsFinder` structure
    ///
    /// # Arguments
    ///
    /// * `poly` - polynomial whose roots have to be found.
    pub(super) fn new(poly: Poly<T>) -> Self {
        let der = poly.derive();

        // Set the initial root approximation.
        let initial_guess = init(&poly);

        debug_assert!(poly.degree().unwrap_or(0) == initial_guess.len());

        Self {
            poly,
            der,
            solution: initial_guess,
            iterations: 30,
        }
    }

    /// Define the maximum number of iterations
    ///
    /// # Arguments
    ///
    /// * `iterations` - maximum number of iterations.
    pub(super) fn with_max_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    /// Algorithm to find all the complex roots of a polynomial.
    /// Iterative method that finds roots simultaneously.
    ///
    /// O. Aberth, Iteration Methods for Finding all Zeros of a Polynomial Simultaneously,
    /// Math. Comput. 27, 122 (1973) 339–344.
    ///
    /// D. A. Bini, Numerical computation of polynomial zeros by means of Aberth’s method,
    /// Baltzer Journals, June 5, 1996
    ///
    /// D. A. Bini, L. Robol, Solving secular and polynomial equations: A multiprecision algorithm,
    /// Journal of Computational and Applied Mathematics (2013)
    ///
    /// W. S. Luk, Finding roots of real polynomial simultaneously by means of Bairstow's method,
    /// BIT 35 (1995), 001-003
    pub(super) fn roots_finder(mut self) -> Vec<Complex<T>>
    where
        T: Float,
    {
        let n_roots = self.solution.len();
        let mut done = vec![false; n_roots];

        for _k in 0..self.iterations {
            if done.iter().all(|&d| d) {
                break;
            }

            for (i, d) in done.iter_mut().enumerate() {
                let solution_i = self.solution[i];
                let n_xki = self.poly.eval(&solution_i) / self.der.eval(&solution_i);
                let a_xki: Complex<T> = self
                    .solution
                    .iter()
                    .enumerate()
                    .filter_map(|(j, s)| {
                        // (index j, j_th solution)
                        if j == i {
                            None
                        } else {
                            let den = solution_i - s;
                            Some(den.inv())
                        }
                    })
                    .sum();

                // Overriding the root before updating the other decrease the time
                // the algorithm converges.
                let new = solution_i - n_xki / (Complex::<T>::one() - n_xki * a_xki);
                *d = if solution_i == new {
                    true
                } else {
                    self.solution[i] = new;
                    false
                };
            }
        }
        self.solution
    }
}

/// Simple initialization of roots
///
/// # Arguments
///
/// * `poly` - polynomial whose roots have to be found.
// #[allow(dead_code)]
// fn init_simple<T>(poly: &Poly<T>) -> Vec<Complex<T>>
// where
//     T: Float + FloatConst + MulAdd<Output = T> + NumCast,
// {
//     // Convert degree from usize to float
//     let n = poly.degree().unwrap_or(1);
//     let n_f = T::from(n).unwrap();

//     // Calculate the center of the circle.
//     let a_n = poly.leading_coeff();
//     let a_n_1 = poly[poly.len() - 2];
//     let c = -a_n_1 / n_f / a_n;

//     // Calculate the radius of the circle.
//     let r = poly.eval(c).abs().powf(n_f.recip());

//     // Pre-compute the constants of the exponent.
//     let phi = T::one() * FloatConst::FRAC_PI_2() / n_f;
//     let tau = (T::one() + T::one()) * FloatConst::PI();

//     let initial: Vec<Complex<T>> = (1..=n)
//         .map(|j| {
//             let j_f = T::from(j).unwrap();
//             let ex = tau * j_f / n_f + phi;
//             let ex = Complex::i() * ex;
//             ex.exp() * r + c
//         })
//         .collect();
//     initial
// }

/// Generate the initial approximation of the polynomial roots.
///
/// # Arguments
///
/// * `poly` - polynomial whose roots have to be found.
///
/// # Panics
///
/// Panics if the conversion from usize to T (float) fails.
fn init<T>(poly: &Poly<T>) -> Vec<Complex<T>>
where
    T: Float + FloatConst + NumCast,
{
    // set = Vec<(k as usize, k as Float, ln(c_k) as Float)>
    let set: Vec<(usize, T, T)> = poly
        .coeffs
        .iter()
        .enumerate()
        .map(|(k, c)| (k, T::from(k).unwrap(), c.abs().ln()))
        .collect();

    // Convex hull
    // ch = Vec<(k as usize, k as Float)>
    let ch = convex_hull_top(&set);

    // r = Vec<(k_(i+1) - k_i as usize, r as Float)>
    let r: Vec<(usize, T)> = ch
        .windows(2)
        .map(|w| {
            // w[1] = k_(i+1), w[0] = k_i
            let tmp = (poly.coeffs[w[0].0] / poly.coeffs[w[1].0]).abs();
            (w[1].0 - w[0].0, tmp.powf((w[1].1 - w[0].1).recip()))
        })
        .collect();

    // Initial values
    let tau = (T::one() + T::one()) * FloatConst::PI();
    let initial: Vec<Complex<T>> = r
        .iter()
        .flat_map(|&(n_k, r)| {
            let n_k_f = T::from(n_k).unwrap();
            (0..n_k).map(move |i| {
                let i_f = T::from(i).unwrap();
                let ex = tau * i_f / n_k_f;
                (Complex::i() * ex).exp() * r
            })
        })
        .collect();
    initial
}

/// Calculate the upper convex hull of the given set of points.
///
/// # Arguments
///
/// * `set` - set of points.
///
/// # Reference
///
/// T. H. Cormen, C. E. Leiserson, R. L. Rivest, C. Stein,
/// Introduction to Algorithms, 3rd edition, McGraw-Hill Education, 2009,
/// A. M. Andrew, "Another Efficient Algorithm for Convex Hulls in Two Dimensions",
/// Info. Proc. Letters 9, 216-219 (1979)
///
/// # Algorithm
///
/// Monotone chain, a.k.a. Andrew's algorithm— O(n log n)
/// The algorithm is a variant of Graham scan which sorts the points
/// lexicographically by their coordinates.
/// https://en.wikipedia.org/wiki/Convex_hull_algorithms
fn convex_hull_top<T>(set: &[(usize, T, T)]) -> Vec<(usize, T)>
where
    T: Clone + Mul<Output = T> + PartialOrd + Sub<Output = T> + Zero,
{
    let mut stack = Vec::<(usize, T, T)>::new();
    stack.push(set[0].clone());
    stack.push(set[1].clone());

    for p in set.iter().skip(2) {
        loop {
            let length = stack.len();
            // There shall be at least 2 elements in the stack.
            if length < 2 {
                break;
            }
            let next_to_top = stack.get(length - 2).unwrap();
            let top = stack.last().unwrap();

            let cp = cross_product(
                (next_to_top.1.clone(), next_to_top.2.clone()),
                (top.1.clone(), top.2.clone()),
                (p.1.clone(), p.2.clone()),
            );
            // Remove the top if it is not a strict turn to the right.
            if cp < T::zero() {
                break;
            } else {
                stack.pop();
            }
        }
        stack.push(p.clone());
    }

    let res: Vec<_> = stack.iter().map(|(a, b, _c)| (*a, b.clone())).collect();
    // res is already sorted by k.
    res
}

/// Compute the cross product of (p1 - p0) and (p2 - p0)
///
/// `(p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y)`
///
/// # Reference
///
/// T. H. Cormen, C. E. Leiserson, R. L. Rivest, C. Stein,
/// Introduction to Algorithms, 3rd edition, McGraw-Hill Education, 2009,
/// paragraph 33.1
fn cross_product<T>(p0: (T, T), p1: (T, T), p2: (T, T)) -> T
where
    T: Clone + Mul<Output = T> + Sub<Output = T>,
{
    let first = (p1.0 - p0.0.clone(), p1.1 - p0.1.clone());
    let second = (p2.0 - p0.0, p2.1 - p0.1);
    first.0 * second.1 - second.0 * first.1
}

/// Calculate the complex roots of the quadratic equation x^2 + b*x + c = 0.
///
/// # Arguments
///
/// * `b` - first degree coefficient
/// * `c` - zero degree coefficient
#[allow(clippy::many_single_char_names)]
pub(super) fn complex_quadratic_roots_impl<T: Float>(b: T, c: T) -> (Complex<T>, Complex<T>) {
    let two = T::one() + T::one();
    let b_ = b / two;
    let d = b_.powi(2) - c; // Discriminant
    let (root1_r, root1_i, root2_r, root2_i) = if d.is_zero() {
        (-b_, T::zero(), -b_, T::zero())
    } else if d.is_sign_negative() {
        // Negative discriminant.
        let s = (-d).sqrt();
        (-b_, -s, -b_, s)
    } else {
        // Positive discriminant.
        let s = d.sqrt();
        let g = if b > T::zero() { T::one() } else { -T::one() };
        let h = -(b_ + g * s);
        (c / h, T::zero(), h, T::zero())
    };

    (
        Complex::new(root1_r, root1_i),
        Complex::new(root2_r, root2_i),
    )
}

/// Calculate the real roots of the quadratic equation x^2 + b*x + c = 0.
///
/// # Arguments
///
/// * `b` - first degree coefficient
/// * `c` - zero degree coefficient
#[allow(clippy::many_single_char_names)]
pub(super) fn real_quadratic_roots_impl<T: Float>(b: T, c: T) -> Option<(T, T)> {
    let two = T::one() + T::one();
    let b_ = b / two;
    let d = b_.powi(2) - c; // Discriminant
    let (r1, r2) = if d.is_zero() {
        (-b_, -b_)
    } else if d.is_sign_negative() {
        return None;
    } else {
        // Positive discriminant.
        let s = d.sqrt();
        let g = if b > T::zero() { T::one() } else { -T::one() };
        let h = -(b_ + g * s);
        (c / h, h)
    };

    Some((r1, r2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterative_roots_finder() {
        let roots = &[10.0_f32, 10. / 323.4, 1., -2., 3.];
        let poly = Poly::new_from_roots(roots);
        let rf = RootsFinder::new(poly);
        let actual = rf.roots_finder();
        assert_eq!(roots.len(), actual.len());
    }
}
