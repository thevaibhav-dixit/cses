use std::io::{self, BufRead, BufWriter, StdinLock, StdoutLock};
use std::str::FromStr;

pub struct Scanner<R: BufRead> {
    reader: R,
    buf: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buf: Vec::new() }
    }

    pub fn next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        loop {
            if let Some(tok) = self.buf.pop() {
                return tok.parse().expect("parse failed");
            }
            let mut line = String::new();
            let n = self.reader.read_line(&mut line).expect("read failed");
            assert!(n != 0, "unexpected EOF");
            self.buf = line.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.next()).collect()
    }

    pub fn line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).expect("read failed");
        line.trim_end_matches('\n').trim_end_matches('\r').to_string()
    }
}

pub fn stdin() -> Scanner<StdinLock<'static>> {
    Scanner::new(io::stdin().lock())
}

pub fn stdout() -> BufWriter<StdoutLock<'static>> {
    BufWriter::new(io::stdout().lock())
}
