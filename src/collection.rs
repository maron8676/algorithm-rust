#[cfg(test)]
mod tests {
    #[test]
    fn binary_search_test() {
        assert_eq!(Ok(1), (vec![2, 4, 5, 9, 10]).binary_search(&4));
        assert_eq!(Err(1), (vec![2, 4, 5, 9, 10]).binary_search(&3));
        assert_eq!(Err(3), (vec![2, 4, 5, 9, 10]).binary_search(&6));
    }

    #[test]
    fn stack_test() {
        let mut stack = vec![];
        stack.push(3);
        stack.push(2);

        assert_eq!(2, stack.len());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(3), stack.pop());
        assert_eq!(None, stack.pop());
        assert_eq!(0, stack.len());
    }
}
