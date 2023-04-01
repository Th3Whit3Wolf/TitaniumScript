mod parse_data;
mod generate;

use crate::{
    ci::{
        generate::gen_summary,
        parse_data::{parse_coverage, parse_junit},
    },
    flags,
};

use std::{env, fs::OpenOptions, io::prelude::*, path::Path};
use xshell::Shell;

impl flags::Ci {
    pub(crate) fn run(self, _sh: &Shell) -> anyhow::Result<()> {
        let workspace_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let junit_path = workspace_path.join("target").join("nextest").join("ci").join("test.xml");
        let coverage_path =
            workspace_path.join("target").join("nextest").join("ci").join("coverage.xml");

        let test_results = parse_junit(junit_path)?;
        let coverage_results = parse_coverage(coverage_path)?;

        let html = gen_summary(test_results, coverage_results);
        match env::var("GITHUB_STEP_SUMMARY") {
            Ok(summary_path) => {
                let mut file =
                    OpenOptions::new().read(true).append(true).create(true).open(summary_path)?;
                file.write_all(html.as_bytes())?;
                file.sync_all()?;
            }
            Err(e) => {
                eprintln!("Unable to find github step summary env. {e}");
            }
        }

        Ok(())
    }
}
