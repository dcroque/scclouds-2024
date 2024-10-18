pub fn functional(p: u64) -> bool {
    if p <= 1 {return false }
    
    let limit = ((p + 2) as f64).powf(0.5_f64).ceil() as u64;
    let test_vec: Vec<u64> = (2..limit).collect();
    test_vec.into_iter().map(|x| p % x).all(|x| x > 0)
}

pub fn recursive(p: u64) -> bool {
    if p <= 1 {return false }

    let limit = ((p + 2) as f64).powf(0.5_f64).ceil() as u64;
    match p {
        _ if p < 4 => true,
        _ => recursive_helper(p, limit, 2),
    }
}

pub fn recursive_helper(p: u64, l: u64, n: u64) -> bool {
    match n {
        _ if n > l => true,
        _ => match p % n {
            0 => false,
            _ => recursive_helper(p, l, n + 1),
        },
    }
}

pub fn linear(p: u64) -> bool {
    if p <= 1 {return false }

    let limit = ((p + 2) as f64).powf(0.5_f64).ceil() as u64;
    for i in 2..limit {
        if p % i == 0 {
            return false;
        }
    }
    true
}

mod tests {
    #[test]
    fn test_functional() {
        let test_func = crate::prime_check::functional;
        assert_eq!(test_func(0), false);
        assert_eq!(test_func(1), false);
        assert_eq!(test_func(2), true);
        assert_eq!(test_func(3), true);
        assert_eq!(test_func(4), false);
        assert_eq!(test_func(11), true);
        assert_eq!(test_func(997), true);
        assert_eq!(test_func(998), false);
    }

    #[test]
    fn test_recursive() {
        let test_func = crate::prime_check::recursive;
        assert_eq!(test_func(0), false);
        assert_eq!(test_func(1), false);
        assert_eq!(test_func(2), true);
        assert_eq!(test_func(3), true);
        assert_eq!(test_func(4), false);
        assert_eq!(test_func(11), true);
        assert_eq!(test_func(997), true);
        assert_eq!(test_func(998), false);
    }

    #[test]
    fn test_linear() {
        let test_func = crate::prime_check::linear;
        assert_eq!(test_func(0), false);
        assert_eq!(test_func(1), false);
        assert_eq!(test_func(2), true);
        assert_eq!(test_func(3), true);
        assert_eq!(test_func(4), false);
        assert_eq!(test_func(11), true);
        assert_eq!(test_func(997), true);
        assert_eq!(test_func(998), false);
    }
}
