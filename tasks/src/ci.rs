mod parse_data;
mod generate;

use crate::{
    ci::{
        generate::gen_summary,
        parse_data::{parse_coverage, parse_junit},
    },
    flags,
};

use std::{env, path::Path};
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
        env::set_var("GITHUB_STEP_SUMMARY", html);
        Ok(())
    }
}
