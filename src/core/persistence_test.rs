#[cfg(test)]
mod persistence_test {
    use crate::core::persistence::{Persistence, save_key_value_pair};
    use crate::core::red_hare::MetaData;

    #[test]
    fn test_set_string_success() {
        let mut vec_data = Vec::<Persistence>::new();
        let mut data = Vec::new();
        data.push(1);
        data.push(2);
        let meta_data = MetaData {
            value: data,
            expire_time: None,
        };
        vec_data.push(Persistence {
            key: "test".to_string(),
            meta_data,
        });
        save_key_value_pair(vec_data)
    }
}
