/// 执行一次基础四则运算，无效运算返回 `None`。
pub fn calculate(left: f64, operator: char, right: f64) -> Option<f64> {
    match operator {
        '+' => Some(left + right),
        '-' => Some(left - right),
        '*' => Some(left * right),
        '/' if right != 0.0 => Some(left / right),
        _ => None,
    }
}

pub fn fibonacci(index: u32) -> u64 {
    let (mut previous, mut current) = (0, 1);

    for _ in 0..index {
        (previous, current) = (current, previous + current);
    }

    previous
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performs_four_operations() {
        assert_eq!(calculate(8.0, '+', 2.0), Some(10.0));
        assert_eq!(calculate(8.0, '-', 2.0), Some(6.0));
        assert_eq!(calculate(8.0, '*', 2.0), Some(16.0));
        assert_eq!(calculate(8.0, '/', 2.0), Some(4.0));
    }

    #[test]
    fn rejects_invalid_operations() {
        assert_eq!(calculate(8.0, '/', 0.0), None);
        assert_eq!(calculate(8.0, '?', 2.0), None);
    }

    #[test]
    fn calculates_fibonacci_numbers() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
