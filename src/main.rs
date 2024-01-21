use csv_util::csv::{BasicReader, Reader};

fn main() {
    let basic_reader = BasicReader::new(String::from("file_1.csv"));
    let lines = basic_reader.read();
    for line in lines {
        println!("{:?}", line);
    }
    println!("Hello, world!");
}
