#![feature(test)]
extern crate test;
extern crate yard;

#[bench]
fn bench_simple_operation(b: &mut test::Bencher) {
    let code = "4 + 3";
    b.iter(|| {
        if let Ok(t) = yard::evaluate::<f64>(code) {
            assert_eq!(t, 7.0);
        }
    });
}

#[bench]
fn bench_complex_operation(b: &mut test::Bencher) {
    let code = "4 + 3 * (1 + 4) + 2 ^ 3";
    b.iter(|| {
        if let Ok(t) = yard::evaluate::<u32>(code) {
            assert_eq!(t, 27);
        }
    });
}

#[bench]
fn bench_big_operation(b: &mut test::Bencher) {
    let code = "3000 + 72000";
    b.iter(|| {
        if let Ok(t) = yard::evaluate::<u32>(code) {
            assert_eq!(t, 75000);
        }
    });
}

#[bench]
fn bench_big_complex_operation(b: &mut test::Bencher) {
    let code = "412 + 34 * (213 + 40 ^ 2) + 122 ^ 3";
    b.iter(|| {
        if let Ok(t) = yard::evaluate::<u32>(code) {
            assert_eq!(t, 1877902);
        }
    });
}

#[bench]
fn bench_nested_operation(b: &mut test::Bencher) {
    let code = "4 * (3 + (7 * (3 - 1) / 2)) + 8)";
    b.iter(|| {
        if let Ok(t) = yard::evaluate::<u32>(code) {
            assert_eq!(t, 48);
        }
    });
}

#[bench]
fn bench_nested_complex_operation(b: &mut test::Bencher) {
    let code = "4 * (3 + (7 * (3 - 1) / 2)) + 8) ^ 2 / 3 + 3.45";
    b.iter(|| {
        if let Ok(t) = yard::evaluate::<u32>(code) {
            assert_eq!(t, 48);
        }
    });
}

#[bench]
fn bench_formatted_operation(b: &mut test::Bencher) {
    let first = "4 * 5";
    let second = "1 + 5";
    let code = format!("{} + {}", first, second);

    b.iter(|| {
        if let Ok(t) = yard::evaluate::<u32>(&code) {
            assert_eq!(t, 26);
        }
    });
}
