use crate::config::log::{load_log_dir, load_manifest};
use crate::core::red_hare::{MetaData, RedHare};
use crate::utils::common::ensure_dir_exists;
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions, read_to_string, rename, write};
use std::io::ErrorKind::Other;
use std::io::{BufWriter, Error};
use std::path::Path;
#[derive(Serialize, Deserialize)]
struct SysManifest {
    rdb_log_info: BasicLog,
    aof_log_list: Vec<BasicLog>,
}
#[derive(Serialize, Deserialize)]
struct AofOperate {
    operate_type: OperateType,
    key: String,
    meta_data: Option<MetaData>,
}
#[derive(Serialize, Deserialize)]
struct BasicLog {
    file_path: String,
    time_stamp: u64,
}

#[derive(Serialize, Deserialize)]
enum OperateType {
    Put,
    Delete,
}

impl RedHare {
    fn append_aof_log(&mut self, aof_log: BasicLog) -> Result<(), Error> {
        let mainfest_path = load_manifest()?;
        let manifest_path_temp = &format!("{}.temp", &mainfest_path);
        let manifest_content_temp = read_to_string(&manifest_path_temp)?;
        let mut old: SysManifest =
            deserialize(&manifest_content_temp.into_bytes()).map_err(|e| Error::new(Other, e))?;
        old.aof_log_list.push(aof_log);
        let new_bytes = serialize(&old).map_err(|e| Error::new(Other, e))?;
        write(&manifest_path_temp, new_bytes)?;
        rename(&manifest_path_temp, &mainfest_path)?;
        Ok(())
    }

    pub fn get_aof_writer(&mut self) -> Result<Option<BufWriter<File>>, Error> {
        let aof_writer = (|| -> Option<BufWriter<File>> {
            let log_dir = load_log_dir().ok()?;
            ensure_dir_exists(&log_dir).ok()?;
            let aof_log_file = Path::new(&log_dir).join("aof_log_file.aof");
            let aof_log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(aof_log_file)
                .ok()?;
            Some(BufWriter::with_capacity(256 * 1024, aof_log_file))
        })();
        Ok(aof_writer)
    }
}
