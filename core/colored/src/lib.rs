use colored::Colorize;

pub enum Color {
    Success(&'static str, &'static str),
    Error(&'static str, &'static str),
}

impl Color {
    pub fn render(&self) {
        match self {
            Color::Success(field, text) => {
                println!("{}: {}", field.green(), text)
            }

            Color::Error(field, error) => {
                panic!("{}: {}", field.red(), error)
            }
        }
    }
}
