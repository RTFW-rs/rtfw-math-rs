pub fn lcm(a: u32, b: u32) -> u32 {
    let base_mul = a * b;
    let mut lcm = base_mul;

    for i in (1..=base_mul).rev() {
        if i % a == 0 && i % b == 0 {
            lcm = i;
        }
    }

    lcm
}

pub fn gcd(a: u32, b: u32) -> u32 {
    let min = if a < b { a } else { b };

    let mut gcd = 1;
    for i in 2..=min {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
    }

    gcd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_8_4() {
        let result = gcd(8, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_gcd_8_5() {
        let result = gcd(8, 5);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_gcd_8_6() {
        let result = gcd(8, 6);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_lcm_8_4() {
        let result = lcm(8, 4);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_lcm_8_5() {
        let result = lcm(8, 5);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_lcm_8_6() {
        let result = lcm(8, 6);
        assert_eq!(result, 24);
    }
}
