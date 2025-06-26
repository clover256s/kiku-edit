use crate::core::language::Language;
use chardet::detect;
use std::{
    fs,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone)]
pub struct Line {
    text: String,
}

impl Line {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len_chars(&self) -> usize {
        self.text.chars().count()
    }

    pub fn width(&self) -> usize {
        self.text.chars().map(|ch| ch.width().unwrap_or(1)).sum()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn mtext(&mut self) -> &mut String {
        &mut self.text
    }
}

pub struct Document {
    pub lines: Vec<Line>,
    pub encoding: String,
    pub language: Language,
    pub file_path: Option<PathBuf>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            lines: vec![Line::new(String::new())],
            encoding: "UTF-8".to_string(),
            language: Language::PlainText,
            file_path: None,
        }
    }

    fn detect_language_from_pathh<P: AsRef<Path>>(&mut self, path: P) {
        if let Some(extension) = path.as_ref().extension() {
            self.language = Language::from(extension.to_string_lossy().as_ref());
        }
    }

    pub fn load<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path_ref = path.as_ref();
        let bytes = match fs::read(path_ref) {
            Ok(bytes) => bytes,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => Vec::new(),
                _ => return Self::new(),
            },
        };

        let (content, encoding) = if let Ok(utf8_str) = String::from_utf8(bytes.clone()) {
            (utf8_str, "UTF-8".to_string())
        } else {
            let (encoding, _, _) = detect(&bytes);
            (String::from_utf8_lossy(&bytes).into_owned(), encoding)
        };

        self.lines = content
            .lines()
            .map(|line| Line::new(line.to_string()))
            .collect();
        if self.lines.is_empty() {
            self.lines.push(Line::new(" ".to_string()));
        }

        self.encoding = encoding;
        self.detect_language_from_pathh(path_ref);
        self.file_path = Some(path_ref.to_path_buf());
        self
    }

    pub fn svae(&self) -> io::Result<()> {
        if let Some(path) = self.file_path.as_ref() {
            let file = fs::File::create(path)?;
            let mut write = BufWriter::new(file);

            for line in &self.lines {
                writeln!(write, "{}", line.text)?;
            }

            write.flush()?;
        }
        Ok(())
    }

    pub fn insert(&mut self, row: usize, col: usize, ch: char) {
        if let Some(line) = self.lines.get_mut(row) {
            if let Some((byte_idx, _)) = line.text.char_indices().nth(col) {
                line.text.insert(byte_idx, ch);
            } else {
                line.text.push(ch);
            }
        }
    }

    pub fn remove(&mut self, row: usize, col: usize) {
        if let Some(line) = self.lines.get_mut(row) {
            if let Some((byte_idx, ch)) = line.text.char_indices().nth(col) {
                line.text.drain(byte_idx..(byte_idx + ch.len_utf8()));
            }
        }
    }

    pub fn remove_line(&mut self, row: usize) -> Line {
        self.lines.remove(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_unicode() {
        let mut doc = Document::new();

        doc.insert(0, 0, 'a');
        assert_eq!(doc.lines[0].text, "a");

        doc.insert(0, 1, '🤣');
        assert_eq!(doc.lines[0].text, "a🤣");

        doc.insert(0, 2, 'b');
        assert_eq!(doc.lines[0].text, "a🤣b");
    }

    #[test]
    fn test_delete_unicode() {
        let mut doc = Document::new();
        doc.lines[0].text = "你好".to_string();

        doc.remove(0, 0);
        assert_eq!(doc.lines[0].text, "好");

        doc.remove(0, 0);
        assert_eq!(doc.lines[0].text, "");
    }
}
