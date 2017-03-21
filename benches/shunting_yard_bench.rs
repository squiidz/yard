#![feature(test)]
extern crate test;
extern crate yard;

#[bench]
fn bench_shunting_yard(b: &mut test::Bencher) {
    let code = "4 + 3 * (1 + 4) + 2 ^ 3";
    b.iter(|| {
        let _ = yard::evaluate(code);
    });
}
