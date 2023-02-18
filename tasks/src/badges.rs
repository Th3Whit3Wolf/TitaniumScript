use crate::{flags, project_root};
use std::{
    fmt,
    fs::{self, File},
    io::Write,
    process::Command,
};
use xshell::Shell;

use badge_maker::BadgeBuilder;
use colorgrad::Color;

#[derive(Debug, Clone, Copy)]
struct TestSummary {
    passed: u64,
    failed: u64,
    skipped: u64,
    runs: u64,
    total: u64,
}

impl TestSummary {
    // Example output from `cargo nextest run 2>&1 | grep "Summary"`
    // `     Summary [   0.058s] 79/540 tests run: 67 passed, 12 failed, 0 skipped`
    fn try_get() -> Option<TestSummary> {
        let mut passed = None;
        let mut failed = None;
        let mut skipped = None;
        let mut runs = None;
        let mut total = None;

        let output = Command::new("cargo")
            .args(["nextest", "run"])
            .output()
            .expect("failed to execute process");
        let output_err = std::str::from_utf8(&output.stderr).expect("Unable to get stderr");
        let output_out = std::str::from_utf8(&output.stdout).expect("Unable to get stdout");

        let mut output_split: Option<String> = None;
        if output_err.contains("Summary") {
            output_split = Some(
                output_err
                    .split('\n')
                    .filter(|s| s.trim().starts_with("Summary"))
                    .collect::<String>(),
            );
        }
        if output_out.contains("Summary") {
            output_split = Some(
                output_out
                    .split('\n')
                    .filter(|s| s.trim().starts_with("Summary"))
                    .collect::<String>(),
            );
        }
        if let Some(out) = output_split {
            let datas = out.trim().split(' ').collect::<Vec<&str>>();

            for idx in 0..datas.len() {
                match datas[idx] {
                    "tests" => {
                        let total_run_data: Vec<u64> = datas[idx - 1]
                            .split('/')
                            .filter_map(|s| s.parse::<u64>().ok())
                            .collect();
                        if total_run_data.len() == 2 {
                            (runs, total) = (Some(total_run_data[0]), Some(total_run_data[1]));
                        }
                    }
                    "passed," => {
                        let passed_num = datas[idx - 1].parse::<u64>();
                        if let Ok(num) = passed_num {
                            passed = Some(num)
                        }
                    }
                    "failed," => {
                        let failed_num = datas[idx - 1].parse::<u64>();
                        if let Ok(num) = failed_num {
                            failed = Some(num)
                        }
                    }
                    "skipped" => {
                        let skipped_num = datas[idx - 1].parse::<u64>();
                        if let Ok(num) = skipped_num {
                            skipped = Some(num)
                        }
                    }
                    _ => {}
                }
            }
        }
        if let (Some(passed), Some(failed), Some(skipped), Some(total), Some(runs)) =
            (passed, failed, skipped, total, runs)
        {
            Some(TestSummary { passed, failed, skipped, total, runs })
        } else {
            None
        }
    }

    fn fmt_test_badge(&self) -> String {
        format!("✓ {} ✗ {} ➟ {})", self.passed, self.failed, self.skipped)
    }
    fn fmt_sucessful_test_badge(&self) -> String {
        format!(
            "{}% ({}/{})",
            (self.runs as f64 / self.total as f64 * 100.0).round(),
            self.runs,
            self.total
        )
    }
}

fn mk_badges(test_badge_data: String, successful_test_badge_data: String) {
    let badge_data_dir = project_root().join("badge_data");
    if fs::create_dir_all(badge_data_dir.clone()).is_ok() {
        let test_badge_filename = badge_data_dir.join("test_badge.svg");
        let mut test_badge_file = File::create(test_badge_filename)
            .expect("Unable to create badge_data/test_badge.svg directory in project root");
        test_badge_file
            .write_all(test_badge_data.as_bytes())
            .expect("Unable to write to badge_data/test_badge.svg directory in project root");

        let successful_test_badge_filename = badge_data_dir.join("successful_test_badge.svg");
        let mut successful_test_badge_file = File::create(successful_test_badge_filename).expect(
            "Unable to create badge_data/successful_test_badge.svg directory in project root",
        );
        successful_test_badge_file.write_all(successful_test_badge_data.as_bytes()).expect(
            "Unable to write to badge_data/successful_test_badge.svg directory in project root",
        );
    };
}

impl fmt::Display for TestSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}/{} ✓ {} ✗ {} ➟ {})",
            self.runs, self.total, self.passed, self.failed, self.skipped
        )
    }
}

fn color_from_percent(perc: f64) -> String {
    let grad = colorgrad::CustomGradient::new()
        .colors(&[
            Color::from_rgba8(224, 93, 68, 255),
            Color::from_rgba8(223, 179, 23, 255),
            Color::from_rgba8(164, 166, 29, 255),
            Color::from_rgba8(151, 202, 0, 255),
        ])
        .domain(&[0.0, 0.8, 0.9, 1.0])
        .build()
        .unwrap();

    grad.at(perc).to_hex_string()
}

impl flags::Badges {
    pub(crate) fn run(self, _sh: &Shell) -> anyhow::Result<()> {
        if let Some(ts) = TestSummary::try_get() {
            //let mut perc = 0.0;
            let perc = ts.passed as f64 / ts.total as f64;
            let color = color_from_percent(perc);

            let tests_shield = BadgeBuilder::new()
                .label("Tests")
                .message(&ts.fmt_test_badge())
                .color_parse(&color)
                .build()
                .expect("Unable to create tests badge")
                .svg();

            let successful_tests_shield = BadgeBuilder::new()
                .label("Successful Tests")
                .message(&ts.fmt_sucessful_test_badge())
                .color_parse(&color)
                .build()
                .expect("Unable to create successful tests badge")
                .svg();

            println!(
                "Test: {}\nSuccessful Tests: {}\n\n{}\n\n{}",
                ts.fmt_test_badge(),
                ts.fmt_sucessful_test_badge(),
                tests_shield,
                successful_tests_shield
            );

            mk_badges(tests_shield, successful_tests_shield);
        } else {
            println!("Test Summary not found");
        }

        Ok(())
    }
}
