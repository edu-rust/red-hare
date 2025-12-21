#[cfg(test)]
mod config_test {
    use crate::config::config::load_config;

    #[test]
    fn test_load_config() {
        let config = load_config();
        assert!((config.is_ok()));
        assert_eq!(config.unwrap().logging.log_rdb_dir,"./redhare_snapshot.rdb");
    }
}
