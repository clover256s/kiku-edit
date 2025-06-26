use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Rust,
    Python,
    PlainText,
    Other(String),
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::C => "c".to_string(),
            Language::Cpp => "cpp".to_string(),
            Language::Rust => "Rust".to_string(),
            Language::Python => "Python".to_string(),
            Language::PlainText => "txt".to_string(),
            Language::Other(s) => s.to_string(),
        };

        write!(f, "{}", s)?;
        Ok(())
    }
}

impl Language {
    pub fn from(ext: &str) -> Self {
        match ext {
            "c" => Language::C,
            "rs" => Language::Rust,
            "py" => Language::Python,
            "txt" => Language::PlainText,
            "cpp" | "cc" | "cxx" | "h" | "hpp" => Language::Cpp,
            _ => Self::Other(ext.to_string()),
        }
    }
}
