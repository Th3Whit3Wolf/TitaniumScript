use std::{fs, path::Path};

use similar_asserts::assert_eq;

fn lines_with_ends(text: &str) -> LinesWithEnds<'_> {
    LinesWithEnds { text }
}

struct LinesWithEnds<'a> {
    text: &'a str,
}

impl<'a> Iterator for LinesWithEnds<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.text.is_empty() {
            return None;
        }
        let idx = self.text.find('\n').map_or(self.text.len(), |it| it + 1);
        let (res, next) = self.text.split_at(idx);
        self.text = next;
        Some(res)
    }
}

pub fn expect_eq(mut expect: &str, actual: &str) {
    if expect.starts_with('\n') {
        expect = &expect[1..];
    }
    let indent = expect
        .lines()
        .filter(|it| !it.trim().is_empty())
        .map(|it| it.len() - it.trim_start().len())
        .min()
        .unwrap_or(0);
    let trimmed: String = lines_with_ends(expect)
        .map(
            |line| {
                if line.len() <= indent {
                    line.trim_start_matches(' ')
                } else {
                    &line[indent..]
                }
            },
        )
        .collect();
    assert_eq!(&trimmed, actual)
}

pub fn expect_file<P: AsRef<Path>>(path: P, expect: &str) {
    let file = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Unable to read file {}", path.as_ref().display()))
        .replace("\r\n", "\n");
    assert_eq!(&file, expect);
}
