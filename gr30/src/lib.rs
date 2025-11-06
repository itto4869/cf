#![allow(dead_code)]

pub mod io {
    use std::io::{self, Read};

    pub struct Scanner {
        buf: Vec<u8>,
        idx: usize,
    }
    impl Scanner {
        #[inline]
        pub fn new() -> Self {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s).unwrap();
            Self { buf: s.into_bytes(), idx: 0 }
        }
        #[inline]
        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            self.skip_blanks();
            let start = self.idx;
            while self.idx < self.buf.len() && !self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
            } 
            // Safety: we always split on UTF-8 boundaries (ASCII whitespace)
            let s = unsafe { std::str::from_utf8_unchecked(&self.buf[start..self.idx]) };
            match s.parse::<T>() {
                Ok(v) => v,
                Err(_) => panic!("parse error: '{}'", s),
            }
        }
        #[inline]
        pub fn bytes(&mut self, n: usize) -> &[u8] {
            self.skip_blanks();
            let start = self.idx;
            let end = (start + n).min(self.buf.len());
            self.idx = end;
            &self.buf[start..end]
        }
        #[inline]
        fn skip_blanks(&mut self) {
            while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
            }
        }
    }
}