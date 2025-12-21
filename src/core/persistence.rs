use crate::config::config::{Config, load_config};
use crate::core::red_hare::{MetaData, RedHare};
use crate::utils::date::is_after_now;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Persistence {
    key: String,
    meta_data: MetaData,
}
pub fn save_rdb_file() {
    let red_hare = RedHare::single_instance();
    let keys = red_hare.key_set();
    if keys.is_empty() {
        return;
    }
    let mut data_vec = Vec::with_capacity(keys.len());

    for key in keys {
        match red_hare.get_bytes_value_with_expire(key.clone()) {
            Ok(value) => match value {
                None => {}
                Some(meta_data) => match is_after_now(meta_data.expire_time) {
                    Ok(is_after_now) => {
                        if (!is_after_now) {
                            data_vec.push(Persistence { key, meta_data });
                        }
                    }
                    Err(err) => {
                        todo!()
                    }
                },
            },
            Err(err) => {
                todo!()
            }
        }
    }
    if data_vec.is_empty() {
        return;
    }
    save_key_value_pair(data_vec)
}

fn save_key_value_pair(data: Vec<Persistence>) {
    let serial_data = match bincode::serialize(&data) {
        Ok(serial_data) => serial_data,
        Err(error) => {
            todo!()
        }
    };
    let log_rdb_dir = match load_config() {
        Ok(log_rdb_dir) => log_rdb_dir.logging.log_rdb_dir,
        Err(error) => {
            todo!()
        }
    };
    let mut file = match File::create(log_rdb_dir) {
        Ok(file) => file,
        Err(error) => {
            todo!()
        }
    };
    match file.write_all(&serial_data) {
        Ok(ok) => {
            //todo!()
        }
        Err(error) => {
            todo!()
        }
    }
    drop(file)
}
