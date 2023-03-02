// // //! This module greps parser's code for specially formatted comments and turns
// // //! them into tests.

// use std::{
//     fs, mem,
//     path::{Path, PathBuf},
// };

// use std::{collections::HashMap, iter};

// pub(crate) fn project_root() -> PathBuf {
//     let dir = env!("CARGO_MANIFEST_DIR");
//     let res = PathBuf::from(dir).parent().unwrap().parent().unwrap().to_owned();
//     assert!(res.join("rustfmt.toml").exists());
//     res
// }

// fn normalize_newlines(s: &str) -> String {
//     s.replace("\r\n", "\n")
// }

// /// Checks that the `file` has the specified `contents`. If that is not the
// /// case, updates the file and then fails the test.
// pub(crate) fn ensure_file_contents(file: &Path, contents: &str) {
//     if let Ok(old_contents) = fs::read_to_string(file) {
//         if normalize_newlines(&old_contents) == normalize_newlines(contents) {
//             // File is already up to date.
//             return;
//         }
//     }

//     let display_path = file.strip_prefix(project_root()).unwrap_or(file);
//     eprintln!(
//         "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
//         display_path.display()
//     );
//     if std::env::var("CI").is_ok() {
//         eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
//     }
//     if let Some(parent) = file.parent() {
//         let _ = fs::create_dir_all(parent);
//     }
//     fs::write(file, contents).unwrap();
//     panic!("some file was not up to date and has been updated, simply re-run the tests");
// }

// #[derive(Debug)]
// struct Test {
//     name: String,
//     text: String,
//     ok: bool,
// }

// #[derive(Default, Debug)]
// struct Tests {
//     ok: HashMap<String, Test>,
//     err: HashMap<String, Test>,
// }

// #[derive(Clone)]
// struct CommentBlock {
//     // id: String,
//     line: usize,
//     contents: Vec<String>,
//     is_doc: bool,
// }

// impl CommentBlock {
//     fn extract_untagged(text: &str) -> Vec<CommentBlock> {
//         let mut res = Vec::new();

//         let lines = text.lines().map(str::trim_start);

//         let dummy_block = CommentBlock {
//             /* id: String::new(), */ line: 0,
//             contents: Vec::new(),
//             is_doc: false,
//         };
//         let mut block = dummy_block.clone();
//         for (line_num, line) in lines.enumerate() {
//             match line.strip_prefix("//") {
//                 Some(mut contents) => {
//                     if let Some('/' | '!') = contents.chars().next() {
//                         contents = &contents[1..];
//                         block.is_doc = true;
//                     }
//                     if let Some(' ') = contents.chars().next() {
//                         contents = &contents[1..];
//                     }
//                     block.contents.push(contents.to_string());
//                 }
//                 None => {
//                     if !block.contents.is_empty() {
//                         let block = mem::replace(&mut block, dummy_block.clone());
//                         res.push(block);
//                     }
//                     block.line = line_num + 2;
//                 }
//             }
//         }
//         if !block.contents.is_empty() {
//             res.push(block);
//         }
//         res
//     }
// }

// fn collect_tests(s: &str) -> Vec<Test> {
//     let mut res = Vec::new();
//     for comment_block in CommentBlock::extract_untagged(s) {
//         let first_line = &comment_block.contents[0];
//         let (name, ok) = if let Some(name) = first_line.strip_prefix("test ") {
//             (name.to_string(), true)
//         } else if let Some(name) = first_line.strip_prefix("test_err ") {
//             (name.to_string(), false)
//         } else {
//             continue;
//         };
//         let text: String = comment_block.contents[1..]
//             .iter()
//             .cloned()
//             .chain(iter::once(String::new()))
//             .collect::<Vec<_>>()
//             .join("\n");
//         assert!(!text.trim().is_empty() && text.ends_with('\n'));
//         res.push(Test { name, text, ok })
//     }
//     res
// }

// fn list_files(dir: &Path) -> Vec<PathBuf> {
//     let mut res = Vec::new();
//     let mut work = vec![dir.to_path_buf()];
//     while let Some(dir) = work.pop() {
//         for entry in dir.read_dir().unwrap() {
//             let entry = entry.unwrap();
//             let file_type = entry.file_type().unwrap();
//             let path = entry.path();
//             let is_hidden =
//                 path.file_name().unwrap_or_default().to_str().unwrap_or_default().starts_with('.');
//             if !is_hidden {
//                 if file_type.is_dir() {
//                     work.push(path);
//                 } else if file_type.is_file() {
//                     res.push(path);
//                 }
//             }
//         }
//     }
//     res
// }

// fn list_rust_files(dir: &Path) -> Vec<PathBuf> {
//     let mut res = list_files(dir);
//     res.retain(|it| {
//         it.file_name().unwrap_or_default().to_str().unwrap_or_default().ends_with(".rs")
//     });
//     res
// }
// fn tests_from_dir(dir: &Path) -> Tests {
//     let mut res = Tests::default();
//     for entry in list_rust_files(dir) {
//         process_file(&mut res, entry.as_path());
//     }
//     let grammar_rs = dir.parent().unwrap().join("grammar.rs");
//     process_file(&mut res, &grammar_rs);
//     return res;

//     fn process_file(res: &mut Tests, path: &Path) {
//         let text = fs::read_to_string(path).unwrap();

//         for test in collect_tests(&text) {
//             if test.ok {
//                 if let Some(old_test) = res.ok.insert(test.name.clone(), test) {
//                     panic!("Duplicate test: {}", old_test.name);
//                 }
//             } else if let Some(old_test) = res.err.insert(test.name.clone(), test) {
//                 panic!("Duplicate test: {}", old_test.name);
//             }
//         }
//     }
// }

// fn existing_tests(dir: &Path, ok: bool) -> HashMap<String, (PathBuf, Test)> {
//     let mut res = HashMap::default();
//     for file in fs::read_dir(dir).unwrap() {
//         let file = file.unwrap();
//         let path = file.path();
//         if path.extension().unwrap_or_default() != "rs" {
//             continue;
//         }
//         let name = {
//             let file_name = path.file_name().unwrap().to_str().unwrap();
//             file_name[5..file_name.len() - 3].to_string()
//         };
//         let text = fs::read_to_string(&path).unwrap();
//         let test = Test { name: name.clone(), text, ok };
//         if let Some(old) = res.insert(name, (path, test)) {
//             println!("Duplicate test: {old:?}");
//         }
//     }
//     res
// }

// #[test]
// fn gen_parser_tests() {
//     let grammar_dir = project_root().join(Path::new("crates/crates_tests/src/grammar"));
//     let tests = tests_from_dir(&grammar_dir);

//     install_tests(&tests.ok, "crates/crates_tests/test_data/parser/inline/ok");
//     install_tests(&tests.err, "crates/crates_tests/test_data/parser/inline/err");

//     fn install_tests(tests: &HashMap<String, Test>, into: &str) {
//         let tests_dir = project_root().join(into);
//         if !tests_dir.is_dir() {
//             fs::create_dir_all(&tests_dir).unwrap();
//         }
//         // ok is never actually read, but it needs to be specified to create a Test in existing_tests
//         let existing = existing_tests(&tests_dir, true);
//         for t in existing.keys().filter(|&t| !tests.contains_key(t)) {
//             panic!("Test is deleted: {t}");
//         }

//         let mut new_idx = existing.len() + 1;
//         for (name, test) in tests {
//             let path = match existing.get(name) {
//                 Some((path, _test)) => path.clone(),
//                 None => {
//                     let file_name = format!("{new_idx:04}_{name}.rs");
//                     new_idx += 1;
//                     tests_dir.join(file_name)
//                 }
//             };
//             ensure_file_contents(&path, &test.text);
//         }
//     }
// }
