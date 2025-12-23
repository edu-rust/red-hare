#[cfg(test)]
mod persistence_test {
    use crate::storage::persistence::{
        restore_rdb_file, save_rdb_file,
    };
    use crate::core::red_hare::RedHare;

    //测试之前,需要删除旧的rdb文件
    #[test]
    fn save_rdb_file_test() {
        let red_hare = RedHare::singleton();
        let ret = red_hare.set_string("test_key1".to_string(), "test_value1".to_string());
        assert!(ret.is_ok());

        let ret = red_hare.set_string("test_key2".to_string(), "test_value2".to_string());
        assert!(ret.is_ok());

        let ret = red_hare.set_string("test_key3".to_string(), "test_value3".to_string());
        assert!(ret.is_ok());
        save_rdb_file();
    }

    #[test]
    fn restore_rdb_file_test() {
        restore_rdb_file();
        let red_hare = RedHare::singleton();
        let ret = red_hare.get_string("test_key1".to_string());
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().unwrap(), "test_value1".to_string());

        let ret = red_hare.get_string("test_key2".to_string());
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().unwrap(), "test_value2".to_string());

        let ret = red_hare.get_string("test_key3".to_string());
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().unwrap(), "test_value3".to_string());
    }
}
