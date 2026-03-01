pub fn fibo(x: u32) -> u32 {
    match x {
        0 => 0,
        1 => 1,
        x => fibo(x - 1) + fibo(x - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibo_0() {
        assert_eq!(fibo(0), 0);
    }

    #[test]
    fn test_fibo_1() {
        assert_eq!(fibo(1), 1);
    }

    #[test]
    fn test_fibo_2() {
        assert_eq!(fibo(2), 1);
    }

    #[test]
    fn test_fibo_3() {
        assert_eq!(fibo(3), 2);
    }

    #[test]
    fn test_fibo_4() {
        assert_eq!(fibo(4), 3);
    }

    #[test]
    fn test_fibo_5() {
        assert_eq!(fibo(5), 5);
    }

    #[test]
    fn test_fibo_6() {
        assert_eq!(fibo(6), 8);
    }

    #[test]
    fn test_fibo_7() {
        assert_eq!(fibo(7), 13);
    }
}
