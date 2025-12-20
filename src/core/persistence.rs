use crate::core::red_hare::RedHare;

pub fn save_rdb_file() {
    let red_hare = RedHare::single_instance();
    let keys = red_hare.key_set();
    if keys.is_empty() {
        return;
    }
    for key in keys {
        match red_hare.get_bytes_value(key.clone()) {
            Ok(value) => match value {
                None => {}
                Some(value) => {
                    save_key_value_pair(key.as_str(), value);
                }
            },
            Err(err) => {}
        }
    }
}

pub fn save_key_value_pair(key: &str, value: Vec<u8>) {

}
