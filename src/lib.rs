
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
        pub path: Option<String>,
        pub extension: Option<String>,
        pub delimiter: Option<char>,
    }

    impl FileInfo {
        fn new(path: Option<String>, extension: Option<String>, delimiter: Option<char>) -> Self {
            Self { path, extension, delimiter }
        }
        fn default() -> Self {
            Self { path: None, extension: None, delimiter: Some(',') }
        }
    }

    pub struct BasicReader {
        // This is a struct that contains information about the file
        file_info: Option<FileInfo>,
    }

    impl BasicReader {
        pub fn new(path: String) -> Self {
            let file_info = Self::get_file_info(&path);
            return Self { file_info: Some(file_info) };
        }
        fn get_file_info(path: &str) -> FileInfo {
            let path = Path::new(path);
            FileInfo {
                path: Some(path.file_name().map(|name| name.to_str().unwrap().to_string()).unwrap_or_default()),
                extension: Some(path.extension().map(|ext| ext.to_str().unwrap().to_string()).unwrap_or_default()),
                delimiter: Some(','),
            }
        }
        /// Check whether the given file exists
        fn exists(&self) -> bool {
            let binding = self.file_info.as_ref().unwrap().path.as_ref().unwrap();
            let path = Path::new(&binding);
            path.exists()
        }
    }

    impl Reader for BasicReader {
        fn read(&self) -> Vec<Vec<String>> {
            let mut output = Vec::new();
            let file = File::open(&self.file_info.as_ref().unwrap().path.as_ref().unwrap()).unwrap();
            let lines = BufReader::new(file).lines();
            for line in lines {
                let line = line.unwrap();
                let mut row = Vec::new();
                // TODO: Replace this with a delimiter that is specified by the user.
                // By default, this value should be a comma.
                for column in line.split(',') {
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
