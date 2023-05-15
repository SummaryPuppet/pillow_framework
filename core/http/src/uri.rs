/// Http URI
///
/// # Examples
///
/// ```rust
/// Uri("/usrs/01".to_string())
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Uri(pub String);

impl Uri {
    /// Get the value in Uri
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
