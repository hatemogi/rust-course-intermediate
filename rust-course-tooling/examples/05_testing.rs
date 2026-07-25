fn checked_divide(left: i32, right: i32) -> Option<i32> {
    (right != 0).then(|| left / right)
}

fn main() {
    assert_eq!(checked_divide(12, 3), Some(4));
}

#[cfg(test)]
mod tests {
    use super::checked_divide;

    #[test]
    fn divides_two_integers() {
        assert_eq!(checked_divide(12, 3), Some(4));
        assert_ne!(checked_divide(12, 3), Some(5));
    }

    #[test]
    fn returns_none_when_divisor_is_zero() {
        assert!(checked_divide(12, 0).is_none());
    }
}
