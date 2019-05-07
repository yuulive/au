extern crate automatica;

use automatica::linear_system::{self, Ss};
use automatica::transfer_function::TfMatrix;
use automatica::Eval;
use nalgebra::{Complex, DMatrix, DVector};

fn main() {
    let a = DMatrix::from_row_slice(2, 2, &[-1., 1., -1., 0.25]);
    let b = DMatrix::from_row_slice(2, 1, &[1., 0.25]);
    let c = DMatrix::from_row_slice(2, 2, &[0., 1., -1., 1.]);
    let d = DMatrix::from_row_slice(2, 1, &[0., 1.]);

    let sys = Ss::new(a.clone(), b.clone(), c.clone(), d.clone());
    let poles = sys.poles();

    println!("{}", &sys);
    println!("poles: {}", poles);

    println!("Equilibrium for u=0");
    let eq = sys.equilibrium(&[0.]);
    println!("{}", eq.unwrap());

    let (pc, a_inv) = linear_system::leverrier(&a);
    let g = a_inv.left_mul(&c).right_mul(&b);
    let rest = pc.clone() * d;
    println!("pc: {}\n(sI-A)^-1: {}\n", &pc, &a_inv);
    let G = g + rest;
    println!("G:{}", G);

    let tf_vec = TfMatrix::new(G, pc.clone());
    println!("Tf:{}", tf_vec);

    let u = DVector::from_element(1, Complex::new(0.0, 0.9));
    let ynum = tf_vec.eval(u.clone());
    println!("u:{}\ny:{}", &u, &ynum);

    println!("\nExample of LeVerrier algorithm (Wikipedia)");
    let t = DMatrix::from_row_slice(3, 3, &[3., 1., 5., 3., 3., 1., 4., 6., 4.]);
    let (p, poly_matrix) = linear_system::leverrier(&t);
    println!("T: {}\np: {}\n", &t, &p);
    println!("B: {}", &poly_matrix);
}
