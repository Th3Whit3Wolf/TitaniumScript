use std::{fs, path::Path};

use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Debug)]
pub(crate) struct NextestOverview {
    pub(crate) count: u64,
    pub(crate) time: f64,
    pub(crate) errors: u64,
    pub(crate) failures: u64,
}

#[derive(Debug)]
pub(crate) struct NextestSuite {
    pub(crate) name: String,
    // pub(crate) count: u64,
    // pub(crate) disabled: u64,
    // pub(crate) errors: u64,
    // pub(crate) failures: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct NextestTest {
    pub(crate) name: String,
    pub(crate) suite: String,
    pub(crate) time: f64,
    pub(crate) failure: bool,
}

#[derive(Debug)]
pub struct NextestResult {
    pub(crate) overview: NextestOverview,
    pub(crate) suites: Vec<NextestSuite>,
    pub(crate) tests: Vec<NextestTest>,
}

#[derive(Debug)]
pub(crate) struct CoverageOverview {
    pub(crate) branches_covered: u64,
    pub(crate) branches_total: u64,
    pub(crate) branch_coverage: u64,
    pub(crate) complexity: u64,
    pub(crate) lines_covered: u64,
    pub(crate) lines_total: u64,
    pub(crate) line_coverage: f64,
}

#[derive(Debug)]
pub(crate) struct CoverageFileAnalysis {
    pub(crate) path: String,
    // pub(crate) branch_coverage: u64,
    // pub(crate) complexity: u64,
    pub(crate) line_coverage: f64,
}

#[derive(Debug)]
pub struct CoverageResult {
    pub(crate) overview: CoverageOverview,
    pub(crate) files: Vec<CoverageFileAnalysis>,
}

pub(crate) fn parse_junit<P>(junit_path: P) -> anyhow::Result<NextestResult>
where
    P: AsRef<Path>,
{
    let xml = fs::read_to_string(junit_path)?;
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut suites: Vec<NextestSuite> = Vec::new();
    let mut tests: Vec<NextestTest> = Vec::new();
    let mut overview = NextestOverview { count: 0, time: 0.0, errors: 0, failures: 0 };

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"testsuites" => {
                    e.attributes().for_each(|maybe_attr| {
                        if let Ok(attr) = maybe_attr {
                            let key_name = attr.key.local_name();
                            let val = &String::from_utf8(attr.value.into()).unwrap();

                            match key_name.as_ref() {
                                b"tests" => overview.count = val.parse::<u64>().unwrap(),
                                b"time" => overview.time = val.parse::<f64>().unwrap(),
                                b"errors" => overview.errors = val.parse::<u64>().unwrap(),
                                b"failures" => overview.failures = val.parse::<u64>().unwrap(),
                                _ => (),
                            }
                        }
                    });
                }
                b"testsuite" => {
                    let mut name = String::new();
                    let mut count = 0;
                    let mut disabled = 0;
                    let mut errors = 0;
                    let mut failures = 0;

                    e.attributes().for_each(|maybe_attr| {
                        if let Ok(attr) = maybe_attr {
                            let key_name = attr.key.local_name();
                            let val = &String::from_utf8(attr.value.into()).unwrap();
                            let n = val.parse::<u64>();

                            match key_name.as_ref() {
                                b"name" => name = val.to_owned(),
                                b"tests" => count = n.unwrap(),
                                b"disabled" => disabled = n.unwrap(),
                                b"errors" => errors = n.unwrap(),
                                b"failures" => failures = n.unwrap(),
                                _ => (),
                            }
                        }
                    });

                    // suites.push(NextestSuite { name, count, disabled, errors, failures });
                    suites.push(NextestSuite { name });
                }

                b"testcase" => {
                    let mut name = String::new();
                    let mut suite = String::new();
                    let mut time = 0.0;

                    e.attributes().for_each(|maybe_attr| {
                        if let Ok(attr) = maybe_attr {
                            let key_name = attr.key.local_name();
                            let val = &String::from_utf8(attr.value.into()).unwrap();

                            match key_name.as_ref() {
                                b"name" => name = val.to_owned(),
                                b"classname" => suite = val.to_owned(),
                                b"time" => time = val.parse::<f64>().unwrap(),
                                _ => (),
                            }
                        }
                    });

                    tests.push(NextestTest { name, suite, time, failure: false });
                }
                b"failure" => {
                    if let Some(last) = tests.last_mut() {
                        *last = NextestTest {
                            name: last.name.to_owned(),
                            suite: last.suite.to_owned(),
                            time: last.time,
                            failure: true,
                        };
                    }
                }
                _ => (),
            },

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    Ok(NextestResult { overview, suites, tests })
}

pub(crate) fn parse_coverage<P>(coverage_path: P) -> anyhow::Result<CoverageResult>
where
    P: AsRef<Path>,
{
    let xml = fs::read_to_string(coverage_path)?;
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut files: Vec<CoverageFileAnalysis> = Vec::new();
    let mut overview = CoverageOverview {
        branches_covered: 0,
        branches_total: 0,
        branch_coverage: 0,
        complexity: 0,
        lines_covered: 0,
        lines_total: 0,
        line_coverage: 0.0,
    };

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"coverage" => {
                    e.attributes().for_each(|maybe_attr| {
                        if let Ok(attr) = maybe_attr {
                            let key_name = attr.key.local_name();
                            let val = &String::from_utf8(attr.value.into()).unwrap();

                            match key_name.as_ref() {
                                b"branches-covered" => {
                                    overview.branches_covered = val.parse::<u64>().unwrap()
                                }
                                b"branches-valid" => {
                                    overview.branches_total = val.parse::<u64>().unwrap()
                                }
                                b"branch-rate" => {
                                    overview.branch_coverage = val.parse::<u64>().unwrap()
                                }
                                b"complexity" => overview.complexity = val.parse::<u64>().unwrap(),
                                b"lines-covered" => {
                                    overview.lines_covered = val.parse::<u64>().unwrap()
                                }
                                b"lines-valid" => {
                                    overview.lines_total = val.parse::<u64>().unwrap()
                                }
                                b"line-rate" => {
                                    overview.line_coverage = val.parse::<f64>().unwrap() * 100.0
                                }
                                _ => (),
                            }
                        }
                    });
                }
                b"class" => {
                    let mut path = String::new();
                    let mut branch_coverage = 0;
                    let mut complexity = 0;
                    let mut line_coverage = 0.0;

                    e.attributes().for_each(|maybe_attr| {
                        if let Ok(attr) = maybe_attr {
                            let key_name = attr.key.local_name();
                            let val = &String::from_utf8(attr.value.into()).unwrap();

                            match key_name.as_ref() {
                                b"branch-rate" => branch_coverage = val.parse::<u64>().unwrap(),
                                b"line-rate" => line_coverage = val.parse::<f64>().unwrap() * 100.0,
                                b"complexity" => complexity = val.parse::<u64>().unwrap(),
                                b"filename" => {
                                    path = val.split("titaniumscript").last().unwrap().to_owned();
                                }
                                _ => (),
                            }
                        }
                    });

                    files.push(CoverageFileAnalysis {
                        path,
                        // branch_coverage,
                        // complexity,
                        line_coverage,
                    });
                }
                _ => (),
            },

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    Ok(CoverageResult { overview, files })
}
