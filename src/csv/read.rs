use serde::de::DeserializeOwned;

/// 读取csv数据，解析成结构体
/// @param path - csv文件路径
/// @param has_header - 是否有表头
/// @return 结果向量
pub fn read<D>(path: &str, has_header: bool) -> Result<Vec<D>, std::io::Error>
where
    D: DeserializeOwned,
{
    // 初始化结果向量
    let mut result = Vec::new();
    // 初始化csv读取器
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(has_header)
        .from_path(path)?;
    for row in rdr.deserialize() {
        let row: D = row?;
        result.push(row);
    }

    Ok(result)
}
