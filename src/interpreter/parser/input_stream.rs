use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;

pub struct InputStream<'a> {
    line: u32,
    col: u32,
    data: Peekable<Chars<'a>>
}

impl<'a> InputStream<'a> {
    pub fn from_string(d: &'a String) -> InputStream<'a> {
        let test = d.chars().into_iter().peekable().clone();
        InputStream { line: 1, col: 1, data: test }
    }
    pub fn get_line(&self) -> u32 { self.line }
    pub fn get_col(&self) -> u32 { self.col }
    pub fn next(&mut self) -> Option<char> {
        match self.data.next() {
            Some(ch) => {
                if ch == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                Some(ch)
            },
            None => {
                None
            }
        }
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.data.peek()
    }
    pub fn is_eof(&mut self) -> bool {
        self.data.peek().is_none()
    }
}