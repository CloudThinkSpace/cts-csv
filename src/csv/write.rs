use std::{fs::{create_dir_all, File}, path::Path};

use csv::Writer;
use serde::Serialize;

/// 写入数据到指定的文件路径
pub fn write<S>(path: &str, filename: &str, data: &[S]) -> Result<(), std::io::Error>
where
    S: Serialize,
{
    create_dir_all(path)?;
    let file = File::create(format!("{}/{}", path, filename))?;
    let mut writer = Writer::from_writer(file);

    for row in data {
        writer.serialize(row)?;
    }

    Ok(())
}


/// 写入数据到指定的文件路径
pub fn write_file<S>(path: &str, data: &[S]) -> Result<(), std::io::Error>
where
    S: Serialize,
{
    let (parent_path, _) = split_path_strings(path);
    if let Some(path) = parent_path {
         create_dir_all(path)?;
    }
    let file = File::create(path)?;
    let mut writer = Writer::from_writer(file);

    for row in data {
        writer.serialize(row)?;
    }

    Ok(())
}

fn split_path_strings(path: &str) -> (Option<String>, Option<String>) {
    let path_obj = Path::new(path);
    let parent = path_obj.parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string());
    let file_name = path_obj.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string());

    (parent, file_name)
}
