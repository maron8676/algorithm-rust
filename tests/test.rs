extern crate algorithm_rust;

#[test]
fn test_max() {
    assert_eq!(Some(3), algorithm_rust::basic::max(vec![1, 2, 3]));
}
