use csv_util::csv::{BasicReader, Reader};

fn main() {
    let basic_reader = BasicReader::new(String::from("liense"));
    let lines = basic_reader.read();
    for line in lines {
        println!("{:?}", line);
    }
    println!("Hello, world!");
}
