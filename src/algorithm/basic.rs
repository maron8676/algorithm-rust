#[cfg(test)]
mod tests {
    #[test]
    fn swap_test() {
        let mut a = 1;
        let mut b = 2;
        std::mem::swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn max_test() {
        assert_eq!(2, std::cmp::max(1, 2));
        assert_eq!(2, std::cmp::max(2, 2));
        assert_eq!(3, std::cmp::max(2, 3));
    }

    #[test]
    fn sum_test() {
        assert_eq!(10, vec![1, 2, 3, 4].iter().sum());

        assert_eq!(
            Some(10),
            vec![Some(1), Some(2), Some(3), Some(4)]
                .iter()
                .copied()
                .sum()
        );
        assert_eq!(
            None as Option<i32>,
            vec![Some(1), Some(2), None, Some(4)].iter().copied().sum()
        );
        
        assert_eq!(
            Ok(10) as Result<i32, &str>,
            vec![Ok(1), Ok(2), Ok(3), Ok(4)]
                .iter()
                .copied()
                .sum()
        );
        assert_eq!(
            Err("error") as Result<i32, &str>,
            vec![Ok(1), Ok(2), Err("error"), Ok(4)].iter().copied().sum()
        );
    }
}
