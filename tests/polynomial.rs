extern crate automatica;
#[macro_use]
extern crate approx;

use automatica::{poly, Eval, Poly};
use num_traits::{One, Zero};

#[test]
fn arithmetics() {
    let p1 = poly!(1, 1, 1);
    let p2 = poly!(-1, -1, -1);
    let result = p1 + p2;
    assert_eq!(Poly::<i32>::zero(), result);

    let p3 = poly!(1., 1., 1., 1., 1.);
    let p4 = poly!(-1., 0., 1.);
    let quotient = &p3 / &p4;
    let reminder = &p3 % &p4;
    assert_eq!(poly!(2., 1., 1.), quotient);
    assert_eq!(poly!(3., 2.), reminder);

    let original = p4.mul_fft(quotient) + reminder;
    assert_eq!(p3.degree(), original.degree());
    for i in 0..=original.degree().unwrap() {
        assert_relative_eq!(p3[i], original[i]);
    }
}

#[test]
fn maximum_minimum() {
    let cubic = Poly::<f32>::new_from_roots(&[-1., 0., 1.]);
    let slope = cubic.derive();
    let mut stationary = slope.real_roots().unwrap();
    stationary.sort_by(|x, y| x.partial_cmp(y).unwrap());

    // Test roots of derivative.
    assert_relative_eq!(-0.57735, stationary[0], max_relative = 1e-5);
    assert_relative_eq!(0.57735, stationary[1], max_relative = 1e-5);

    let curvature = slope.derive();

    // Local maximum.
    assert!(curvature.eval(stationary[0]).is_sign_negative());
    // Local minimum.
    assert!(curvature.eval(stationary[1]).is_sign_positive());
}

#[test]
fn roots_consistency() {
    // Wilkinson's polynomial.
    let wp = Poly::new_from_roots(&[
        1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    ]);

    // Roots with Aberth-Ehrlich Method.
    let mut roots = wp.iterative_roots();
    roots.sort_by(|&x, &y| x.re.partial_cmp(&y.re).unwrap());

    // Roots with eigenvalue decomposition.
    let mut roots2 = wp.complex_roots();
    roots2.sort_by(|&x, &y| x.re.partial_cmp(&y.re).unwrap());

    for (i, e) in roots.iter().zip(roots2) {
        assert_relative_eq!(i.re, e.re, max_relative = 1e-2);
        assert_relative_eq!(i.im, e.im);
    }
}

#[test]
fn chebyshev_first_kind() {
    // Recurrence relation:
    // T0(x) = 1
    // T1(x) = x
    // T_{n+1}(x) = 2xT_n(x) - T_{n-1}(x)
    let mut polys: Vec<Poly<i32>> = Vec::new();
    polys.push(Poly::<i32>::one());
    polys.push(poly!(0, 1));
    let c = poly!(0, 2);
    for n in 2..12 {
        let tmp = &c * &polys[n - 1];
        polys.push(&tmp - &polys[n - 2]);
    }

    let t2 = poly!(-1, 0, 2);
    assert_eq!(t2, polys[2]);

    let t3 = poly!(0, -3, 0, 4);
    assert_eq!(t3, polys[3]);

    let t4 = poly!(1, 0, -8, 0, 8);
    assert_eq!(t4, polys[4]);

    let t5 = poly!(0, 5, 0, -20, 0, 16);
    assert_eq!(t5, polys[5]);

    let t6 = poly!(-1, 0, 18, 0, -48, 0, 32);
    assert_eq!(t6, polys[6]);

    let t7 = poly!(0, -7, 0, 56, 0, -112, 0, 64);
    assert_eq!(t7, polys[7]);

    let t8 = poly!(1, 0, -32, 0, 160, 0, -256, 0, 128);
    assert_eq!(t8, polys[8]);

    let t9 = poly!(0, 9, 0, -120, 0, 432, 0, -576, 0, 256);
    assert_eq!(t9, polys[9]);

    let t10 = poly!(-1, 0, 50, 0, -400, 0, 1120, 0, -1280, 0, 512);
    assert_eq!(t10, polys[10]);

    let t11 = poly!(0, -11, 0, 220, 0, -1232, 0, 2816, 0, -2816, 0, 1024);
    assert_eq!(t11, polys[11]);
}
