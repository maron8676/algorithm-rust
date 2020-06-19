fn max<T>(v: Vec<T>) -> Option<T>
where
    T: Ord + Copy,
{
    v.iter().copied().max()
}

fn arg_max<T>(v: Vec<T>) -> Option<usize>
where
    T: Ord + Copy,
{
    v.iter()
        .enumerate()
        .max_by_key(|&(_index, value)| value)
        .map_or(None, |(index, _)| Some(index))
}

fn min<T>(v: Vec<T>) -> Option<T>
where
    T: Ord + Copy,
{
    v.iter().copied().min()
}

fn arg_min<T>(v: Vec<T>) -> Option<usize>
where
    T: Ord + Copy,
{
    v.iter()
        .enumerate()
        .min_by_key(|&(_index, value)| value)
        .map_or(None, |(index, _)| Some(index))
}

fn sum<T>(v: Vec<T>) -> T
where
    T: std::iter::Sum + Copy,
{
    v.iter().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(Some(4), max(vec![2, 4, 3, 1]));
        assert_eq!(None as Option<i32>, max(vec![]));

        assert_eq!(Some(1), arg_max(vec![2, 4, 3, 1]));
        assert_eq!(None as Option<usize>, arg_max(vec![] as Vec<i32>));
    }

    #[test]
    fn min_test(){
        assert_eq!(1, std::cmp::min(1, 2));
        assert_eq!(2, std::cmp::min(2, 2));
        assert_eq!(2, std::cmp::min(2, 3));

        assert_eq!(Some(1), min(vec![2, 4, 3, 1]));
        assert_eq!(None as Option<i32>, min(vec![]));

        assert_eq!(Some(3), arg_min(vec![2, 4, 3, 1]));
        assert_eq!(None as Option<usize>, arg_min(vec![] as Vec<i32>));
    }

    #[test]
    fn sum_test() {
        assert_eq!(10, sum(vec![1, 2, 3, 4]));

        assert_eq!(Some(10), sum(vec![Some(1), Some(2), Some(3), Some(4)]));
        assert_eq!(
            None as Option<i32>,
            sum(vec![Some(1), Some(2), None, Some(4)])
        );

        assert_eq!(
            Ok(10) as Result<i32, &str>,
            sum(vec![Ok(1), Ok(2), Ok(3), Ok(4)])
        );
        assert_eq!(
            Err("error") as Result<i32, &str>,
            sum(vec![Ok(1), Ok(2), Err("error"), Ok(4)])
        );
    }
}
