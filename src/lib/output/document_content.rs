use std::{cmp, env, fs, io};
use std::path::Path;

pub struct DocumentContent {
    lines: Vec<Box<str>>,
}

impl DocumentContent {
    pub fn new() -> Self {
        let mut args = env::args();

        match args.nth(1) {
            None => Self {
                lines: Vec::new()
            },
            Some(file) => Self::from_file(file.as_ref()),
        }
    }

    fn from_file(file: &Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        Self {
            lines: file_contents.lines().map(|line| line.into()).collect(),
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.lines.len()
    }

    pub fn get_row(&self, at: usize) -> &str {
        &self.lines[at]
    }

    pub fn empty(&self) -> bool {
        self.number_of_rows() == 0
    }
}

impl<Idx> std::ops::Index<Idx> for DocumentContent 
where
    Idx: std::slice::SliceIndex<[Box<str>]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.lines[index]
    }
}