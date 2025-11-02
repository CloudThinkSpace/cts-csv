# 读写csv文件
## read 读取数组
```rust
  let path = "data/test.csv";
  let data = read::<Vec<String>>(path,true).expect("msg");
  println!("{:?}", data);
```

## read_header 读取表头数据
```rust
    let path = "data/test.csv";
    let data = read_header::<String>(path).expect("msg");
    println!("{:?}", data);
```

## read 读取数组
```rust
  let path = "data/test.csv";
  let data = read::<Vec<String>>(path,true).expect("msg");
  println!("{:?}", data);
```

## read 读取对象
```rust
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    pub struct Row {
        #[serde(rename(deserialize = "ID", serialize = "序号"))]
        pub id: usize,
        #[serde(rename(deserialize = "BSM", serialize = "地块编号"))]
        pub bsm: String,
    }
    let path = "data/test.csv";
    let data = read::<Row>(path,true).expect("msg");
    println!("{:?}", data);
```
## write 写入数组
```rust
  let path = "data/test.csv";
  let data = vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]];
  write::<Vec<String>>(path, data).expect("msg");
```

## write 写入对象
```rust
  let path = "data/test.csv";
  let data = vec![Row{id:1,bsm:"1".to_string()}];
  write::<Row>(path, data).expect("msg");
```
