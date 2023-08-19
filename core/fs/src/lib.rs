use mime_guess::MimeGuess;
use std::{fs, io::Read, path::Path};

pub struct FS {}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub metadata: fs::Metadata,
    pub file_type: fs::FileType,
    pub content: Option<Vec<u8>>,
    pub content_type: Option<String>,
    pub buffer: Option<Vec<u8>>,
}

impl File {
    pub fn from_dir_entry(path: String, dir_entry: &fs::DirEntry) -> File {
        let mut content: Option<Vec<u8>> = None;
        let metadata = dir_entry.metadata().unwrap();
        let mut content_type: Option<String> = None;
        let buffer: Option<Vec<u8>> = None;

        if metadata.clone().is_file() {
            let is_other_type_extension = if let Some(extension) = path.rsplit('.').next() {
                !(extension.to_lowercase() == "html") || !(extension.to_lowercase() == "hbs")
            } else {
                false
            };

            if is_other_type_extension {
                content = Some(FS::read_to_buf(&path))
            } else {
                content = Some(FS::read_to_string(&path));
            }

            content_type = Some(FS::get_mime_type(path.as_str()));
        }

        File {
            path,
            metadata,
            file_type: dir_entry.file_type().unwrap().clone(),
            content,
            content_type,
            buffer,
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
        let err_msg = format!("No such file or directory ' {} '", path);

        let directory_root = fs::read_dir(path).expect(&err_msg);
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
    pub fn read_to_hex(path: &str) -> String {
        let mut file = fs::File::open(path).expect("No se pudo abrir el archivo");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Error al leer el archivo");

        let hex_string: String = buffer.iter().map(|byte| format!("{:02X}", byte)).collect();

        hex_string
    }

    pub fn read_to_buf(path: &str) -> Vec<u8> {
        let mut file = fs::File::open(path).expect("No se pudo abrir el archivo");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Error al leer el archivo");

        buffer
    }

    pub fn read_to_string(path: &str) -> Vec<u8> {
        match fs::read_to_string(path) {
            Ok(content) => content.as_bytes().into(),
            Err(_) => FS::read_to_buf(path),
        }
    }

    fn get_mime_type(path: &str) -> String {
        let path = Path::new(path);

        MimeGuess::from_path(path)
            .first_or_octet_stream()
            .to_string()
    }
}
