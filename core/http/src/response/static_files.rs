use pillow_fs::FS;

#[derive(Debug)]
#[allow(dead_code)]
pub struct StaticFiles {
    pub files: Vec<StaticFile>,
}

#[derive(Debug, Clone)]
pub struct StaticFile {
    // pub kind: StaticFileType,
    pub route_absolute: String,
    pub path: String,
    pub name_file: String,
    pub content: Vec<u8>,
    pub lenght: usize,
    file_original: pillow_fs::File,
}

#[derive(Debug, Clone)]
pub enum StaticFileType {
    CSS,
    JS,
}

impl Default for StaticFiles {
    fn default() -> Self {
        Self::new("resources")
    }
}

impl StaticFiles {
    #[allow(dead_code)]
    pub fn new(directory: &str) -> StaticFiles {
        let directory_files = FS::get_all_in_directories(directory);

        let vec_files = Self::normalize(directory_files);

        let mut files: Vec<StaticFile> = Vec::new();
        for f in vec_files {
            files.push(StaticFile::new(f.clone()));
        }

        StaticFiles { files }
    }

    pub fn normalize(directory: Vec<pillow_fs::File>) -> Vec<pillow_fs::File> {
        let mut files: Vec<pillow_fs::File> = Vec::new();
        let directories: Vec<pillow_fs::File> = directory.clone();

        fn lop(directories: &[pillow_fs::File], files: &mut Vec<pillow_fs::File>) {
            for f in directories {
                if f.file_type.is_dir() {
                    let d = FS::get_all_in_directories(&f.path);
                    lop(&d, files);
                }

                if f.file_type.is_file() {
                    files.push(f.clone())
                }
            }
        }

        lop(&directories, &mut files);

        files
    }
}

/*
impl StaticFiles {
    pub fn get_css_files(&self) -> Vec<StaticFile> {
        let mut css: Vec<StaticFile> = Vec::new();

        for c in &self.files {
            match c.kind {
                StaticFileType::CSS => css.push(c.clone()),
                StaticFileType::JS => {}
            }
        }

        css
    }

    pub fn get_javascript_files(&self) -> Vec<StaticFile> {
        let mut js: Vec<StaticFile> = Vec::new();

        for j in &self.files {
            match j.kind {
                StaticFileType::CSS => {}
                StaticFileType::JS => js.push(j.clone()),
            }
        }

        js
    }
}
*/

impl StaticFile {
    pub fn new(file: pillow_fs::File) -> StaticFile {
        // let re = Regex::new(r"[a-zA-Z]+\.[a-zA-Z]+");
        // let regex = re.unwrap();

        let route_absolute = format!("{}", &file.path);

        let content = file.clone().content.unwrap();

        let path = format!("/{}", &route_absolute);

        StaticFile {
            // kind,
            route_absolute: route_absolute.to_string(),
            path,
            name_file: route_absolute.to_string(),
            content: content.clone(),
            lenght: content.len(),
            file_original: file.clone(),
        }
    }
}

impl StaticFile {
    pub fn metadata(&self) -> std::fs::Metadata {
        self.file_original.metadata.clone()
    }

    pub fn file_type(&self) -> std::fs::FileType {
        self.file_original.file_type.clone()
    }

    pub fn content_type(&self) -> Option<String> {
        self.file_original.clone().content_type
    }
}
