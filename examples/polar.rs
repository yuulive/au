extern crate automatica;

use automatica::{
    plots::polar::PolarPlot, polynomial::Poly, transfer_function::Tf, RadiantsPerSecond,
};

fn main() {
    let tf = Tf::new(
        Poly::new_from_coeffs(&[5.]),
        Poly::new_from_roots(&[-1., -10.]),
    );

    println!("T:\n{}", tf);

    let p = tf.polar(RadiantsPerSecond(0.1), RadiantsPerSecond(10.0), 0.1);
    for g in p {
        println!(
            "r: {:.3}, i: {:.3}, f: {:.3}, m: {:.3}",
            g.real(),
            g.imag(),
            g.phase(),
            g.magnitude()
        );
    }
}
