use crate::prime_check;

pub fn functional(p: u64) -> Vec<u64> {
    match p {
        _ if p <= 1 => Vec::new(),
        _ if p == 2 => Vec::from([2]),
        _ => (2..(p + 1))
            .collect::<Vec<u64>>()
            .into_iter()
            .filter(|x| prime_check::functional(*x))
            .collect::<Vec<u64>>(),
    }
}

pub fn recursive(p: u64) -> Vec<u64> {
    match p {
        _ if p <= 1 => Vec::new(),
        _ if p == 2 => Vec::from([2]),
        _ => recursive_helper(p, 3, Vec::from([2])),
    }
}

pub fn recursive_helper(p: u64, n: u64, mut result: Vec<u64>) -> Vec<u64> {
    match n {
        _ if n > p => result,
        _ => {
            match result.iter().map(|x| n % x).any(|x| x == 0) {
                true => (),
                false => result.push(n),
            };
            recursive_helper(p, n + 1, result)
        }
    }
}

pub fn linear(p: u64) -> Vec<u64> {
    match p {
        _ if p <= 1 => Vec::new(),
        _ if p == 2 => Vec::from([2]),
        _ => {
            let mut result = Vec::from([2]);
            for i in 3..p + 1 {
                let mut is_prime = true;
                for n in result.as_slice() {
                    match i % n {
                        0 => {
                            is_prime = false;
                            continue;
                        }
                        _ => (),
                    }
                }
                if is_prime {
                    result.push(i);
                }
            }
            result
        }
    }
}

mod tests {
    #[test]
    fn test_functional() {
        let test_func = crate::prime_find::functional;
        assert_eq!(test_func(2), Vec::from([2]));
        assert_eq!(test_func(3), Vec::from([2, 3]));
        assert_eq!(test_func(4), Vec::from([2, 3]));
        assert_eq!(test_func(11), Vec::from([2, 3, 5, 7, 11]));
        assert_eq!(
            test_func(50),
            Vec::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47])
        );
    }

    #[test]
    fn test_recursive() {
        let test_func = crate::prime_find::recursive;
        assert_eq!(test_func(2), Vec::from([2]));
        assert_eq!(test_func(3), Vec::from([2, 3]));
        assert_eq!(test_func(4), Vec::from([2, 3]));
        assert_eq!(test_func(11), Vec::from([2, 3, 5, 7, 11]));
        assert_eq!(
            test_func(50),
            Vec::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47])
        );
    }

    #[test]
    fn test_linear() {
        let test_func = crate::prime_find::linear;
        assert_eq!(test_func(2), Vec::from([2]));
        assert_eq!(test_func(3), Vec::from([2, 3]));
        assert_eq!(test_func(4), Vec::from([2, 3]));
        assert_eq!(test_func(11), Vec::from([2, 3, 5, 7, 11]));
        assert_eq!(
            test_func(50),
            Vec::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47])
        );
    }
}
