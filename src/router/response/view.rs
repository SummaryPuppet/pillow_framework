use std::{fs, path::PathBuf};

pub struct View {
    path: String,
}

impl View {
    /// Returns a View instance
    pub fn new() -> View {
        View {
            path: String::from("resources"),
        }
    }
}

impl View {
    pub fn _render(&self) {}

    /// Returns page in String format
    pub fn render_page(&self, page: String) -> String {
        let path = format!("{}/views/{page}.html", &self.path);
        let content = fs::read_to_string(path);

        match content {
            Ok(file) => return file,
            Err(_) => {
                panic!("Error: Create directory resources/views in root");
            }
        }
    }

    /// Returns static files in Tuple(String,String) format
    pub fn static_files(&self) -> (String, String) {
        let directories = match fs::read_dir(&self.path) {
            Ok(directory) => directory,
            Err(_) => panic!("Error: Create directory resources/css and resources/js in root"),
        };

        let mut static_dirs = Vec::new();

        for dir in directories {
            match dir {
                Ok(directories) => static_dirs.push(directories),
                Err(error) => panic!("{error}"),
            }
        }

        let css = dir(static_dirs[1].path());
        let js = dir(static_dirs[2].path());

        (css, js)
    }
}

fn dir(directory: PathBuf) -> String {
    let directory = fs::read_dir(directory).unwrap();
    let mut file_path = PathBuf::new();

    for files in directory {
        match files {
            Ok(file) => file_path = file.path(),
            Err(error) => panic!("{error}"),
        }
    }

    let content = fs::read_to_string(file_path);

    match content {
        Ok(content) => content,
        Err(error) => panic!("{error}"),
    }
}
