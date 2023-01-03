use pillow_fs::FS;

use handlebars::Handlebars;
use serde_json::Value;

#[derive(Debug)]
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
    /// Returns page in String format
    pub fn render_page(&self, page: String) -> String {
        let path = format!("{}/views/{}.html", &self.path, page);

        FS::read_to_string(path.as_str())
    }

    /// Returns page in String format
    pub fn render_handlebars(&self, page: String, data: Value) -> String {
        let path = format!("{}/views/{}.hbs", &self.path, page);
        let mut reg = Handlebars::new();

        reg.register_template_file(page.as_str(), path).unwrap();

        reg.render(&page, &data).unwrap()
    }
}
