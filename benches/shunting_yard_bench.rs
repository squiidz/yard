#![feature(test)]
extern crate test;
extern crate yard;

use yard::{eval, parse};

#[bench]
fn bench_shunting_yard(b: &mut test::Bencher) {
    let code = "4 + 3 * (1 + 4)";
    b.iter(|| {
        eval(parse(code).expect("Error generating tokens"));
    });
}