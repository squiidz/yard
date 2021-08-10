extern crate yard;

use yard::{parser, evaluator};

#[test]
fn test_minus() {
    let equa = "3 - 1";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 2);
}

#[test]
fn test_plus() {
    let equa = "3.1 + 1.23";
    let tokens = parser::parse::<f32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 4.33);
}

#[test]
fn test_multiply() {
    let equa = "3 * 3";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 9);
}

#[test]
fn test_divide() {
    let equa = "6 / 2";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 3);
}

#[test]
fn test_paren() {
    let equa = "2 * (1 + 2)";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 6);
}

#[test]
fn test_multi_expressions() {
    let equa = "3 * (1 + 2) - 4 / 2";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 7);
}

#[test]
fn test_paren_as_first_expression() {
    let equa = "(1 + 2) * 2";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 6);
}

#[test]
fn test_evaluate_fn() {
    let equa = "1 + 2 * 4";
    let result = yard::evaluate::<i32>(equa).unwrap();
    assert_eq!(result, 9);
}

#[test]
fn test_multi_digit() {
    let equa = "23 + 42 * 11";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 485);
}

#[test]
fn test_negative_multi_digit() {
    let equa = "-2--3-(-4--7)-5.4";
    let tokens = parser::parse::<f32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -7.4);
}

#[test]
fn test_multi_parens() {
    let equa = "2 * (2 + 3) * (4 + 5)";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 90);
}

#[test]
fn test_nested_parens() {
    let equa = "2 * (2 * (3 + 4))";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 28);
}

#[test]
fn test_deep_stack() {
    let equa = "1 - 2 + 3 * 4 * 5 + 6 - 7 + 8 - 9";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 1 - 2 + 3 * 4 * 5 + 6 - 7 + 8 - 9);
}

#[test]
fn test_error() {
    let equa = "9999999999999999999999 + 235";
    let tokens = parser::parse::<u64>(equa);
    assert!(tokens.is_err());
}