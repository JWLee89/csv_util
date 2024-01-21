
pub mod csv {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::path::Path;

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
    struct FileInfo {
        path: Option<String>,
        extension: Option<String>,
    }

    pub struct BasicReader {
        // TODO: If there are more than one piece of information
        // That is common for readers, try to create a struct
        path: String,
        // This is a struct that contains information about the file
        file_info: Option<FileInfo>,
    }

    impl BasicReader {
        pub fn new(path: String) -> Self {
            let file_info = Self::get_file_info(&path);
            return Self { path, file_info: Some(file_info) };
        }
        fn get_file_info(path: &str) -> FileInfo {
            let path = Path::new(path);
            FileInfo {
                path: Some(path.file_name().map(|name| name.to_str().unwrap().to_string()).unwrap_or_default()),
                extension: Some(path.extension().map(|ext| ext.to_str().unwrap().to_string()).unwrap_or_default()),
            }
        }
        /// Check whether the given file exists
        fn exists(&self) -> bool {
            Path::new(&self.path).exists()
        }
    }

    impl Reader for BasicReader {
        fn read(&self) -> Vec<Vec<String>> {
            let mut output = Vec::new();
            let lines = BufReader::new(File::open(&self.path).unwrap()).lines();
            for line in lines {
                let line = line.unwrap();
                let mut row = Vec::new();
                for column in line.split(",") {
                    row.push(column.to_string());
                }
                output.push(row);
            }
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
