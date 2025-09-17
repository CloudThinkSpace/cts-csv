use std::fs::{File, create_dir_all};

use csv::Writer;
use serde::Serialize;

/// 写入数据到指定的文件路径
pub fn write<S>(path: &str, filename: &str, data: &[S]) -> Result<(), std::io::Error>
where
    S: Serialize,
{
    create_dir_all(path)?;
    let file = File::create(format!("{}/{}", path, filename)).expect("Failed to create file");
    let mut writer = Writer::from_writer(file);

    for row in data {
        writer.serialize(row).expect("Failed to serialize row");
    }

    Ok(())
}
