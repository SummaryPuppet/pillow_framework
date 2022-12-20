use crate::http::response::static_files::StaticFiles;

use super::route::Route;

/// Routes
#[derive(Debug)]
pub struct Routes {
    pub get: Vec<Route>,
    pub post: Vec<Route>,
    pub put: Vec<Route>,
    pub delete: Vec<Route>,
}

impl Routes {
    /// New Routes
    pub fn new() -> Routes {
        let st = StaticFiles::new();
        let css = st.get_css_files();
        let javascript = st.get_javascript_files();

        let mut get_static_files = vec![];

        for c in css {
            let content = String::from(&c.content);

            get_static_files.push(Route::new(c.route_absolute, move |_, mut res| {
                res.css(content.clone())
            }));
        }

        for js in javascript {
            let content = String::from(&js.content);

            get_static_files.push(Route::new(js.route_absolute, move |_, mut res| {
                res.javascript(content.clone())
            }));
        }

        Routes {
            get: get_static_files,
            post: Vec::new(),
            put: Vec::new(),
            delete: Vec::new(),
        }
    }
}
