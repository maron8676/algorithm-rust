#[cfg(test)]
mod tests {
    #[test]
    fn binary_search_test() {
        assert_eq!(Ok(1), (vec![2, 4, 5, 9, 10]).binary_search(&4));
        assert_eq!(Err(1), (vec![2, 4, 5, 9, 10]).binary_search(&3));
        assert_eq!(Err(3), (vec![2, 4, 5, 9, 10]).binary_search(&6));
    }
}
