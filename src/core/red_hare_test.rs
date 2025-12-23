#[cfg(test)]
mod red_hare_test {
    use crate::core::red_hare::RedHare;
    use std::time::Duration;

    #[test]
    fn test_set_string_success() {
        let red_hare = RedHare::singleton();
        let result = red_hare.set_string("test_key".to_string(), "test_value".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_set_string_empty_key() {
        let red_hare = RedHare::singleton();
        let result = red_hare.set_string("".to_string(), "test_value".to_string());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "key is empty");
    }

    #[test]
    fn test_get_string_success() {
        let red_hare = RedHare::singleton();
        // 先设置一个键值对
        let result = red_hare.set_string("test_key".to_string(), "test_value".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        // 然后获取它
        let result = red_hare.get_string("test_key".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), "test_value");
    }

    #[test]
    fn test_get_string_with_expire_success() {
        let red_hare = RedHare::singleton();
        let result = red_hare.set_string_with_expire(
            "test_key".to_string(),
            "test_value".to_string(),
            10 * 1_000_000_000u128,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        let result = red_hare.get_string("test_key".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), "test_value");
    }


    #[test]
    fn test_get_string_with_expire_fail() {
        let red_hare = RedHare::singleton();
        let three_seconds= Duration::from_secs(3);
        let result = red_hare.set_string_with_expire(
            "test_key".to_string(),
            "test_value".to_string(),
            three_seconds.as_nanos(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        std::thread::sleep(Duration::from_secs(4));
        let result = red_hare.get_string("test_key".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}





