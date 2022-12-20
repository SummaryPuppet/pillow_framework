#[derive(Debug, Clone, Hash)]
pub struct Json<T>(pub T);

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Json(value)
    }
}

#[macro_export]
macro_rules! json {
    ($($json:tt)+) => (
       serde_json::json!($($json)*);
    );
}
