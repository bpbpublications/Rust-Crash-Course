use tests_2::my_module::add_nums;
use tests_2::my_module::mul_nums;

#[test]
fn test_add_nums() {
    assert_eq!(add_nums(1, 2), 3);
    assert_eq!(add_nums(10, -5), 5);
}

#[test]
fn test_mul_nums() {
    assert_eq!(mul_nums(1, 2), 2);
    assert_eq!(mul_nums(10, 5), 50);
    assert_eq!(mul_nums(10, -2), -20);
    assert_eq!(mul_nums(0, 2), 0);
}