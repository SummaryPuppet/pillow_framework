use regex::Regex;

use crate::fs::FS;

#[derive(Debug)]
pub(crate) struct StaticFiles {
    pub files: Vec<StaticFile>,
}

#[derive(Debug, Clone)]
pub(crate) struct StaticFile {
    pub kind: StaticFileType,
    pub route_absolute: String,
    pub path: String,
    pub name_file: String,
    pub content: String,
    pub lenght: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum StaticFileType {
    CSS,
    JS,
}

impl StaticFiles {
    pub fn new() -> StaticFiles {
        let resources = FS::get_all_in_directories("resources");
        let css = FS::get_all_in_directories(&resources[1].path);
        let js = FS::get_all_in_directories(&resources[2].path);

        let mut content: Vec<StaticFile> = Vec::new();

        for c in css {
            content.push(StaticFile::new(StaticFileType::CSS, &c.path));
        }

        for j in js {
            content.push(StaticFile::new(StaticFileType::JS, &j.path));
        }

        StaticFiles { files: content }
    }
}

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

impl StaticFile {
    pub fn new(kind: StaticFileType, route_absolute: &str) -> StaticFile {
        let re = Regex::new(r"[a-zA-Z]+\.[a-zA-Z]+");
        let regex = re.unwrap();

        let name_file = regex.find(route_absolute).unwrap().as_str();

        let content = FS::read_to_string(route_absolute);

        let path = format!("/{}", &route_absolute);

        StaticFile {
            kind,
            route_absolute: route_absolute.to_string(),
            path,
            name_file: name_file.to_string(),
            content: content.to_string(),
            lenght: content.len(),
        }
    }
}
