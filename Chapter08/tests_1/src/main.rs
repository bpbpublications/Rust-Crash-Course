pub fn add_nums(x:i32, y:i32) -> i32 {
    x + y
}
pub fn mul_nums(x:i32, y:i32) -> i32 {
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_nums() {
        assert_eq!(add_nums(5, 10), 15);
    }

    #[test]
    #[ignore = "Under implementation"]
    fn test_mul_nums() {
        assert_eq!(mul_nums(5, 10), 50);
    }
}

fn main() {
}
