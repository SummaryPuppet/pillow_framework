use handlebars::Handlebars;
use pillow_fs::FS;
use serde_json::Value;
use tera::Tera;

pub use tera::Context;

/// Templates options in Pillow
#[derive(Debug, Clone)]
pub enum Template {
    /// Render Html
    ///
    /// # Arguments
    ///
    /// * `page_name` - Page to render
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow_templates::Template;
    ///
    /// let html = Template::Html("index");
    /// ```
    Html(&'static str),
    /// Render Handlebars
    ///
    /// # Arguments
    ///
    /// * `page_name` - Page to render
    /// * `value` - Value from serde_json
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow_templates::Template;
    /// use serde_json::json;
    ///
    /// let hbs = Template::Handlebars("index", json!({}));
    /// ```
    Handlebars(&'static str, Value),
    /// Render with Tera
    ///
    /// # Arguments
    ///
    /// * `page_name` - Page to render
    /// * `extension` - File extension
    /// * `context` - Context from Tera
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow_templates::Template;
    /// use Tera::Context;
    ///
    /// let ctx = Context::new();
    /// let tera = Template::Tera("index", "html", ctx);
    /// ```
    Tera(&'static str, &'static str, Context),
}

impl Template {
    /// Return a page in string format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pillow_templates::Template;
    ///
    /// let html_template = Template::Html("index");
    //
    /// assert_eq!(html_template.render(), "".to_string());
    /// ```
    pub fn render(&self) -> String {
        let resources = "resources/views";

        match self {
            Template::Html(file) => {
                let path = format!("{}/{}.html", resources, file);

                FS::read_to_string(path.as_str())
            }

            Template::Handlebars(hbs, value) => {
                let path = format!("{}/{}.hbs", resources, hbs);
                let mut reg = Handlebars::new();

                reg.register_template_file(&hbs, path).unwrap();

                reg.render(&hbs, value).unwrap()
            }

            Template::Tera(name_file, ext, ctx) => {
                let path = format!("{}/*", resources);

                let tera = match Tera::new(&path) {
                    Ok(t) => t,
                    Err(e) => panic!("{e}"),
                };

                let template_file = format!("{}.{}", name_file, ext);

                tera.render(&template_file, ctx)
                    .expect("Failed to render template")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_works_html() {
        let page = Template::Html("index_html");

        assert_eq!(page.render(), "");
    }

    #[test]
    fn it_works_hbs() {
        let page = Template::Handlebars("index_hbs", json!({}));

        println!("{:#?}", page.render());

        assert_eq!(page.render(), "");
    }

    #[test]
    fn it_works_tera_html() {
        let ctx = Context::new();
        let page = Template::Tera("index_tera", "html", ctx);

        println!("{:#?}", page.render());

        assert_eq!(page.render(), "");
    }
}
