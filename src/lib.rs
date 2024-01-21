
pub mod csv {

    pub trait Reader {
    fn compare_two_csv_files(&self, other: &BasicReader) -> bool;
        fn read(&self) -> Vec<Vec<String>>;
        fn read_line(&self) -> Vec<String>;
    }

    /// BasicReader is a CSV reader that reads a
    /// CSV file and returns a vector of vectors of strings.
    /// Each vector of strings represents a row in the CSV file.
    /// If the number of columns in a row differs,
    /// we will raise an error to notify users that the CSV file is malformed.
    pub struct BasicReader {
        path: String,
    }

    impl BasicReader {
        pub fn new(path: String) -> Self {
            return Self { path }
        }
        fn file_exists(&self) -> bool {
            return true;

        }
    }

    impl Reader for BasicReader {
        fn read(&self) -> Vec<Vec<String>> {
            let output = Vec::new();
            output
        }

        fn read_line(&self) -> Vec<String> {
            todo!()
        }

        fn compare_two_csv_files(&self, other: &BasicReader) -> bool {
            todo!()
        }
    }

}
