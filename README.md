# 读写csv文件
## read 读取数组
```rust
  let path = "data/test.csv";
  let data = read::<Vec<Vec<String>>>(path,true).expect("msg");
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
    let data = read::<Vec<Row>>(path,true).expect("msg");
    println!("{:?}", data);
```
## write 写入数组
```rust
  let path = "data/test.csv";
  let data = vec![Row{id:1,bsm:"1".to_string()}];
  write::<Vec<Row>>(path, data).expect("msg");
```
