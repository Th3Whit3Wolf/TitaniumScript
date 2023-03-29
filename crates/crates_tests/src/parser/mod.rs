use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use ::parser::{LexedStr, StrStep, SyntaxKind, TopEntryPoint};
use similar_asserts::assert_eq;
use test_utils::expect::expect_file;

#[cfg(test)]
mod lex;

#[allow(clippy::module_inception)]
#[cfg(test)]
mod parser;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TestCase {
    tis: PathBuf,
    tast: PathBuf,
    text: String,
}

impl TestCase {
    fn single(path: &'static str) -> TestCase {
        let crate_root_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("crates_tests");
        let test_data_dir = crate_root_dir.join("test_data");
        let mut tis = test_data_dir.join(path);

        tis.set_extension("tis");
        let tast = tis.with_extension("tast");
        let text = fs::read_to_string(&tis).unwrap();

        if !tis.is_file() {
            panic!("`file` {path}.tis doesn't exist")
        }
        if !tast.is_file() {
            panic!("`file` {path}.tast doesn't exist")
        }

        TestCase { tis, tast, text }
    }
}

fn lex(text: &str) -> String {
    let lexed = LexedStr::new(text);

    let mut res = String::new();
    for i in 0..lexed.len() {
        let kind = lexed.kind(i);
        let text = lexed.text(i);
        let error = lexed.error(i);

        let error = error.map(|err| format!(" error: {err}")).unwrap_or_default();
        if cfg!(target_os = "windows") && kind == SyntaxKind::WHITESPACE {
            writeln!(res, "{kind:?} {:?}{error}", text.replace("\r", "")).unwrap()
        } else {
            writeln!(res, "{kind:?} {text:?}{error}").unwrap()
        }
    }
    res
}

pub(crate) fn check_parser_lex(path: &'static str) {
    let file = TestCase::single(path);
    let actual = lex(&file.text);
    println!("ACTUAL: {actual:?}");
    expect_file(file.tast, &actual);
}

fn parse(entry: TopEntryPoint, text: &str) -> (String, bool) {
    let lexed = LexedStr::new(text);
    let input = lexed.to_input();
    let output = entry.parse(&input);

    let mut buf = String::new();
    let mut errors = Vec::new();
    let mut indent = String::new();
    let mut depth = 0;
    let mut len = 0;
    lexed.intersperse_trivia(&output, &mut |step| match step {
        StrStep::Token { kind, text } => {
            assert!(depth > 0);
            len += text.len();
            writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
        }
        StrStep::Enter { kind } => {
            assert!(depth > 0 || len == 0);
            depth += 1;
            writeln!(buf, "{indent}{kind:?}").unwrap();
            indent.push_str("  ");
        }
        StrStep::Exit => {
            assert!(depth > 0);
            depth -= 1;
            indent.pop();
            indent.pop();
        }
        StrStep::Error { msg, pos } => {
            assert!(depth > 0);
            errors.push(format!("error {pos}: {msg}\n"))
        }
    });
    assert_eq!(
        len,
        text.len(),
        "didn't parse all text.\nParsed:\n{}\n\nAll:\n{}\n",
        &text[..len],
        text
    );

    for (token, msg) in lexed.errors() {
        let pos = lexed.text_start(token);
        errors.push(format!("error {pos}: {msg}\n"));
    }

    let has_errors = !errors.is_empty();
    for e in errors {
        buf.push_str(&e);
    }
    (buf, has_errors)
}

pub(crate) fn check_parser_parser(path: &'static str) {
    let file = TestCase::single(path);
    let (actual, errors) = parse(TopEntryPoint::SourceFile, &file.text);
    if path.contains("err") {
        assert!(errors, "no errors in an ERR file {}\n:{actual}", file.tis.display());
    }

    expect_file(file.tast, &actual);
}
