use std::fs;

pub struct FS {}

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub metadata: fs::Metadata,
    pub file_type: fs::FileType,
}

impl File {
    pub fn from_dir_entry(path: String, dir_entry: &fs::DirEntry) -> File {
        File {
            path,
            metadata: dir_entry.metadata().unwrap(),
            file_type: dir_entry.file_type().unwrap().clone(),
        }
    }
}

impl FS {
    /// Return a Vec for All files in directory
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the file is located
    ///
    /// # Examples
    ///
    /// ```rust
    /// use FS;
    ///
    /// fn (){
    ///     let resources = FS::get_all_in_directories("resources");
    /// }
    /// ```
    pub fn get_all_in_directories(path: &str) -> Vec<File> {
        let directory_root = fs::read_dir(path).unwrap();
        let mut directories_dir_entry = Vec::new();
        let mut directories: Vec<File> = Vec::new();

        for directory in directory_root {
            directories_dir_entry.push(directory.unwrap())
        }

        for directory in &directories_dir_entry {
            let path = String::from(directory.path().clone().to_str().unwrap());

            directories.push(File::from_dir_entry(path, directory));
        }

        directories
    }

    /// Read a file and convert to String
    ///
    /// # Arguments
    ///
    /// * `path` - Path to read file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use FS;
    ///
    /// let file = FS::read_to_string("main.js");
    /// ```
    pub fn read_to_string(path: &str) -> String {
        match fs::read_to_string(path) {
            Ok(content) => content,
            Err(error) => panic!("{}", error),
        }
    }
}
