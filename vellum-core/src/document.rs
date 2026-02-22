use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;


pub struct DocumentBuffer {
    contents: Rope,
}

impl Default for DocumentBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentBuffer {
    pub fn new() -> Self {
        DocumentBuffer { contents: Rope::new() }
    }

    pub fn from_string(text: &str) -> Self {
        let rope = Rope::from_str(text);
        DocumentBuffer { contents: rope }
    }

    pub fn to_string(&self) -> String {
        self.contents.to_string()
    }

    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader).map_err(std::io::Error::other)?;
        Ok(DocumentBuffer { contents: rope })
    }

    pub fn to_file(&self, path: &Path) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.contents
            .write_to(&mut writer)
            .map_err(std::io::Error::other)?;
        Ok(())
    }

    pub fn line_count(&self) -> usize {
        self.contents.len_lines()
    }

    pub fn char_count(&self) -> usize {
        self.contents.len_chars()
    }
}

// TODO: Multi-buffer (for multiple editors)



