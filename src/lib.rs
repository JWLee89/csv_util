
pub mod csv {
    use std::fmt;
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
    #[derive(Debug)]
    struct FileInfo {
        path: Option<String>,
        // Default value should be a ",".
        // Change to \t for tab delimited files.
        delimiter: Option<char>,
    }

    impl FileInfo {
        fn new(path: Option<String>, delimiter: Option<char>) -> Self {
            Self { path, delimiter }
        }
        fn default() -> Self {
            Self { path: None, delimiter: Some(',') }
        }
    }

    impl fmt::Display for FileInfo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Path: {}, Delimiter: {:?}", self.path.as_ref().unwrap(), self.delimiter)
        }
    }

    pub struct BasicReader {
        // This is a struct that contains information about the file
        file_info: Option<FileInfo>,
    }

    impl BasicReader {
        pub fn new(path: String) -> Self {
            let file_info = Self::get_file_info(&path);
            let path_obj = Path::new(&path);
            if !path_obj.exists() {
                panic!("File: \"{path}\" does not exist. \"{file_info}\"", path=path, file_info=file_info);
            }
            match path_obj.extension() {
                Some(ext) => ext,
                None => panic!("File: \"{path}\" does not have an extension. \"{file_info}\"", path=path, file_info=file_info),
            };
            return Self { file_info: Some(file_info) };
        }
        fn get_file_info(path: &str) -> FileInfo {
            let path = Path::new(path);
            FileInfo {
                path: Some(path.file_name().map(|name| name.to_str().unwrap().to_string()).unwrap_or_default()),
                delimiter: Some(','),
            }
        }
        fn is_valid_csv(&self) -> bool {
            todo!()
        }
    }

    impl Reader for BasicReader {
        /// Reads a CSV file and returns a vector of vectors of strings.
        /// This will open the file, read each line, and split each line by a comma.
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
