# 读写csv文件
## read
```rust
  let path = "data/test.csv";
  let data = read::<Vec<Vec<String>>>(path,true).expect("msg");
  println!("{:?}", data);
```
## write
```rust
  let path = "data/test.csv";
  let data = vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]];
  write::<Vec<Vec<String>>>(path, data).expect("msg");
```
