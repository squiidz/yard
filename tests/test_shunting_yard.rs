extern crate yard;

use yard::{parse, eval};

#[test]
fn test_minus() {
    let equa = "3 - 1";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 2);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_plus() {
    let equa = "3 + 1";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 4);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_multiply() {
    let equa = "3 * 3";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 9);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_divide() {
    let equa = "6 / 2";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 3);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_paren() {
    let equa = "2 * (1 + 2)";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 6);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_multi_expressions() {
    let equa = "3 * (1 + 2) - 4 / 2";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 7);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_paren_as_first_expression() {
    let equa = "(1 + 2) * 2";
    match parse::parse(equa) {
        Ok(tokens) => {
            assert_eq!(eval::eval(&tokens), 6);
        },
        Err(_) => panic!(),
    };
}

#[test]
fn test_evaluate_fn() {
    let equa = "1 + 2 * 4";
    let result = yard::evaluate(equa).unwrap();
    assert_eq!(result, 9);
}