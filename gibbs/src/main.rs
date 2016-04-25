extern crate rgsl;

use rgsl::{RngType};
use std::f64;

#[warn(unused_variables)]
fn main() {

    rgsl::RngType::env_setup();
    let t : RngType = rgsl::rng::default();
    let r = rgsl::Rng::new(&t).unwrap();

    println!("Iter x y");

    let mut x = 0.0;
    let mut y = 0.0;

    for i in 0..50000 {
        for thin in 0..1000 {
            x = rgsl::randist::gamma::gamma(&r, 3.0, y*y + 4.0);
            y = 1.0/(x+1.0) + rgsl::randist::gaussian::gaussian(&r, 1.0/(2.0*x+2.0).sqrt());
        }
        println!("{} {} {}", i, x, y);

    }

}
