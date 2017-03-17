extern crate yard;

use yard::{parse, eval};

#[test]
fn test_minus() {
    let equa = "3 - 1";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 2);
}

#[test]
fn test_plus() {
    let equa = "3 + 1";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 4);
}

#[test]
fn test_multiply() {
    let equa = "3 * 3";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 9);
}

#[test]
fn test_divide() {
    let equa = "6 / 2";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 3);
}

#[test]
fn test_paren() {
    let equa = "2 * (1 + 2)";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 6);
}

#[test]
fn test_multi_expressions() {
    let equa = "3 * (1 + 2) - 4 / 2";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 7);
}

#[test]
fn test_paren_as_first_expression() {
    let equa = "(1 + 2) * 2";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 6);
}

#[test]
fn test_evaluate_fn() {
    let equa = "1 + 2 * 4";
    let result = yard::evaluate(equa).unwrap();
    assert_eq!(result, 9);
}

#[test]
fn test_multi_parens() {
    let equa = "2 ^ (2 + 3) * (4 + 5)";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 640);
}

#[test]
fn test_nested_parens() {
    let equa = "2 ^ (2 * (3 + 4))";
    let tokens = parse::parse(equa).unwrap();
    assert_eq!(eval::eval(&tokens), 1<<14);
}