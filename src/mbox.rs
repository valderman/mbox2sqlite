use std::borrow::Borrow;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

#[derive(Debug)]
pub enum MboxError {
    NoSuchFile,
}

pub type Result<T> = std::result::Result<T, MboxError>;

pub struct Mbox {
    lines: Lines<BufReader<File>>,
    next_line: Option<String>,
}

impl Mbox {
    pub fn open(mbox_file_path: &Path) -> Result<Mbox> {
        match File::open(mbox_file_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file).lines();
                let next_line = reader.next().map(|x| x.unwrap());
                Ok(Mbox {
                    lines: reader,
                    next_line: next_line,
                })
            }
            Err(_) => Err(MboxError::NoSuchFile),
        }
    }
}

impl Iterator for Mbox {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_line.borrow() {
            None => None,
            Some(first_line) => {
                let mut message = String::from(first_line);
                self.next_line = None;
                while let Some(line) = self.lines.next() {
                    let ln = line.unwrap();
                    if ln.starts_with("From ") {
                        self.next_line = Some(ln);
                        break;
                    }
                    write!(message, "{}\n", ln).unwrap();
                }
                return Some(message);
            }
        }
    }
}
