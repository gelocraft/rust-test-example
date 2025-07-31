use rust_test_example::is_odd::is_odd;

#[test]
fn test_odd_numbers() {
    assert!(is_odd(1));
    assert!(is_odd(3));
    assert!(is_odd(5));
    assert!(is_odd(7));
}
