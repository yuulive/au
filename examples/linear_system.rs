extern crate automatica;

use automatica::linear_system::Ss;
use automatica::transfer_function::TfMatrix;
use automatica::Eval;

use num_complex::Complex;

fn main() {
    let a = [-1., 1., -1., 0.25];
    let b = [1., 0.25];
    let c = [0., 1., -1., 1.];
    let d = [0., 1.];

    let sys = Ss::new_from_slice(2, 1, 2, &a, &b, &c, &d);
    let poles = sys.poles();

    println!("{}", &sys);
    println!("poles:\n{:?}", poles);

    println!("\nStep response:");
    let iter = sys.rk2(|_| vec![1.], &[0., 0.], 0.1, 150);
    println!("rk2 stationary values: {:?}", iter.last().unwrap());
    // Change to 'true' to print the result
    if false {
        for i in sys.rk2(|_| vec![1.], &[0., 0.], 0.1, 150) {
            println!(
                "{};{};{};{};{}",
                i.time(),
                i.state()[0],
                i.state()[1],
                i.output()[0],
                i.output()[1]
            );
        }
    }

    let rkf45 = sys.rkf45(|_| vec![1.], &[0., 0.], 0.1, 30);
    println!("rkf45 stationary values: {:?}", rkf45.last().unwrap());
    // Change to 'true' to print the result
    if false {
        for i in sys.rkf45(|_| vec![1.], &[0., 0.], 0.1, 30) {
            println!(
                "{};{};{};{};{};{}",
                i.time(),
                i.state()[0],
                i.state()[1],
                i.output()[0],
                i.output()[1],
                i.error()
            );
        }
    }
    let u = 0.0;
    println!("\nEquilibrium for u={}", u);
    let eq = sys.equilibrium(&[u]).unwrap();
    println!("x:\n{:?}\ny:\n{:?}", eq.x(), eq.y());

    println!("\nTransform linear system into a transfer function");
    let tf_matrix = TfMatrix::from(sys);
    println!("Tf:\n{}", tf_matrix);

    println!("\nEvaluate transfer function in ω = 0.9");
    let u = vec![Complex::new(0.0, 0.9)];
    let y = tf_matrix.eval(&u);
    println!("u:\n{:?}\ny:\n{:?}", &u, &y);

    println!(
        "y:\n{:?}",
        &y.iter()
            .map(|x| (x.norm(), x.arg().to_degrees()))
            .collect::<Vec<_>>()
    );
}
