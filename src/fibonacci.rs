pub fn recursive(n: u64) -> Option<u64> {
    match n {
        0 => Some(0),
        1 => Some(1),
        n if n <= 93 => Some(recursive(n - 1).unwrap() + recursive(n - 2).unwrap()),
        _ => None,
    }
}

pub fn binet(n: u64) -> Option<u64> {
    // Binet's Formula
    // Erros de ponto flutuante a partir de binet(12)
    match n {
        n if n <= 93 => Some(
            (1.0 / ((5.0_f64).powf(0.5))
                * (((1.0 + ((5.0_f64).powf(0.5))) / 2.0).powi(n as i32)
                    - ((1.0 - ((5.0_f64).powf(0.5))) / 2.0).powi(n as i32))) as u64,
        ),
        _ => None,
    }
}

pub fn linear(n: u64) -> Option<u64> {
    match n {
        0 => Some(0),
        1 => Some(1),
        n if n <= 93 => {
            let mut i = n;
            let mut result: u64 = 1;
            let mut fib_minus_1: u64 = 0;
            while i > 1 {
                let temp = result;
                result += fib_minus_1;
                fib_minus_1 = temp;
                i -= 1;
            }
            Some(result)
        }
        _ => None,
    }
}

mod tests {
    #[test]
    fn test_recursive() {
        let test_func = crate::fibonacci::recursive;
        assert_eq!(test_func(0), Some(0));
        assert_eq!(test_func(1), Some(1));
        assert_eq!(test_func(2), Some(1));
        assert_eq!(test_func(10), Some(55));
        assert_eq!(test_func(94), None);
        assert_eq!(
            test_func(10),
            Some(test_func(9).unwrap() + test_func(8).unwrap())
        );
    }

    #[test]
    fn test_linear() {
        let test_func = crate::fibonacci::linear;
        assert_eq!(test_func(0), Some(0));
        assert_eq!(test_func(1), Some(1));
        assert_eq!(test_func(2), Some(1));
        assert_eq!(test_func(10), Some(55));
        assert_eq!(test_func(94), None);
        assert_eq!(
            test_func(10),
            Some(test_func(9).unwrap() + test_func(8).unwrap())
        );
    }

    #[test]
    fn test_both() {
        for i in 0..30 {
            assert_eq!(crate::fibonacci::recursive(i), crate::fibonacci::linear(i))
        }
    }
}
