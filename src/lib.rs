pub mod csv;


#[cfg(test)]
mod tests {
    use crate::csv::{read::read, write::write, };

    #[test]
    fn test_read_csv() {
        let path = "data/test.csv";

        let data = read::<Vec<Vec<String>>>(path,true).expect("msg");
        println!("{:?}", data);
    }

    #[test]
    fn test_write_csv() {
        let path = "data/test.csv";

        let data = read::<Vec<Vec<String>>>(path, true).expect("msg");
        write("data", "output.csv", &data).expect("msg");
    }
}
