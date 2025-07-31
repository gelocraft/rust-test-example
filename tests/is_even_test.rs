use rust_test_example::is_even::is_even;

#[test]
fn test_even_numbers() {
    assert!(is_even(2));
    assert!(is_even(4));
    assert!(is_even(6));
}

#[test]
fn test_odd_numbers() {
    assert!(!is_even(1));
    assert!(!is_even(3));
    assert!(!is_even(5));
}
