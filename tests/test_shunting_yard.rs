extern crate yard;

use yard::{evaluator, parser};

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
    assert_eq!(evaluator::eval(&tokens), 57);
}

#[test]
fn test_error() {
    let equa = "9999999999999999999999 + 235";
    let tokens = parser::parse::<u64>(equa);
    assert!(tokens.is_err());
}

#[test]
fn test_power() {
    let equa = "2 ^ 3";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 8);
}

#[test]
fn test_power_with_parens() {
    let equa = "2 ^ (3 + 4)";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 128);
}

#[test]
fn complex_expression() {
    let equa = "2 ^ (3 + 4) * (5 + 6) / 2";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 704);
}

#[test]
fn test_negative_number() {
    let equa = "-2";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -2);
}

#[test]
fn test_negative_number_with_parens() {
    let equa = "(-2)";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -2);
}

#[test]
fn test_negative_number_with_parens_and_power() {
    let equa = "(-2) ^ 3";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -8);
}

#[test]
fn test_negative_number_with_parens_and_power_and_parens() {
    let equa = "(-2) ^ (3 + 4)";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -128);
}

#[test]
fn test_negative_number_with_parens_and_power_2() {
    let equa = "(-2) ^ ((3 + 4))";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -128);
}

#[test]
fn test_negative_number_with_parens_and_power_3() {
    let equa = "(-2) ^ ((3 + 4)) * (5 + 6)";
    let tokens = parser::parse::<i32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), -1408);
}

#[test]
fn test_int_and_float() {
    let equa = "3 + 4.5";
    let tokens = parser::parse::<f32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 7.5);
}

#[test]
fn test_complex_int_and_float() {
    let equa = "3 + 4.5 * 2 - 1 / 2 ^ 2";
    let tokens = parser::parse::<f32>(equa).unwrap();
    assert_eq!(evaluator::eval(&tokens), 11.75);
}
