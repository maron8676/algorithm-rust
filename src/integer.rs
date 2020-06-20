pub fn eratosthenes(n: usize) -> Vec<i32> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes = vec![];
    for num in 2..n + 1 {
        if is_prime[num] {
            primes.push(num as i32);
        }
        for index in (num..n + 1).step_by(num) {
            is_prime[index] = false;
        }
    }
    primes
}

pub fn euclidean(a: i64, b: i64) -> i64 {
    let mut x = a;
    let mut y = b;
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    if x % y == 0 {
        y
    } else {
        euclidean(y, x % y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eratosthenes_test() {
        assert_eq!(vec![] as Vec<i32>, eratosthenes(1));
        assert_eq!(vec![2], eratosthenes(2));
        assert_eq!(vec![2, 3, 5], eratosthenes(5));
    }

    #[test]
    fn euclidean_test() {
        assert_eq!(1, euclidean(1, 4));
        assert_eq!(1, euclidean(4, 1));
        assert_eq!(1, euclidean(3, 5));
        assert_eq!(6, euclidean(6, 12));
    }
}
