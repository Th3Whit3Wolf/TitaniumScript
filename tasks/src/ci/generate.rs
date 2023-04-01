use super::parse_data::{
    CoverageFileAnalysis, CoverageResult, NextestResult, NextestSuite, NextestTest,
};

const PASS_SVG: &str = r##"<svg width="16" height="16" style="margin-top: 2px" class="octicon octicon-check-circle-fill color-fg-success" aria-label="completed successfully" viewBox="0 0 16 16" version="1.1" role="img"><path d="M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16Zm3.78-9.72a.751.751 0 0 0-.018-1.042.751.751 0 0 0-1.042-.018L6.75 9.19 5.28 7.72a.751.751 0 0 0-1.042.018.751.751 0 0 0-.018 1.042l2 2a.75.75 0 0 0 1.06 0Z"></path></svg>"##;
const FAIL_SVG: &str = r##"<svg width="16" height="16" style="margin-top: 2px" class="octicon octicon-x-circle-fill color-fg-danger" aria-label="failed" viewBox="0 0 16 16" version="1.1" role="img"><path d="M2.343 13.657A8 8 0 1 1 13.658 2.343 8 8 0 0 1 2.343 13.657ZM6.03 4.97a.751.751 0 0 0-1.042.018.751.751 0 0 0-.018 1.042L6.94 8 4.97 9.97a.749.749 0 0 0 .326 1.275.749.749 0 0 0 .734-.215L8 9.06l1.97 1.97a.749.749 0 0 0 1.275-.326.749.749 0 0 0-.215-.734L9.06 8l1.97-1.97a.749.749 0 0 0-.326-1.275.749.749 0 0 0-.734.215L8 6.94Z"></path></svg>"##;
pub(crate) trait HTMLTable {
    fn to_th(&self) -> String;
    fn to_td(&self) -> String;
}

impl HTMLTable for NextestSuite {
    fn to_th(&self) -> String {
        "<tr><th>Test Suite</th><th>Passed</th><th>Failed</th></tr>".to_string()
    }
    fn to_td(&self) -> String {
        format!(
            "<tr><td width=\"75%\">{}</td><td>{}</td><td>{}</td></tr>",
            self.name,
            self.count - self.failures,
            self.failures
        )
    }
}

impl HTMLTable for NextestTest {
    fn to_th(&self) -> String {
        "<tr><th>Test</th><th>Time</th><th>Status</th></tr>".to_string()
    }
    fn to_td(&self) -> String {
        format!(
            "<tr><td width=\"75%\">{}</td><td>{}s</td><td>{}</td></tr>",
            self.name,
            self.time,
            if self.failure { FAIL_SVG } else { PASS_SVG }
        )
    }
}

impl HTMLTable for CoverageFileAnalysis {
    fn to_th(&self) -> String {
        "<tr><th>File</th><th>Coverage</th></tr>".to_string()
    }
    fn to_td(&self) -> String {
        format!("<tr><td width=\"75%\">{}</td><td>{}s</td></tr>", self.path, self.line_coverage,)
    }
}

fn summary_details_html(summary: String, details: String) -> String {
    format!(
        "<details>
<summary>{summary}</summary>
{details}
</details>"
    )
}

fn html_table<T: HTMLTable>(t: Vec<T>) -> String {
    if !t.is_empty() {
        let header = t[0].to_th();
        let mut rows = String::new();
        for row in t {
            rows.push_str(&row.to_td())
        }
        format!(
            "<table>
            {header}
            {rows}
            </table>",
        )
    } else {
        String::new()
    }
}

fn test_html(data: NextestResult) -> String {
    let suites_table = html_table(data.suites);
    let tests_summary = summary_details_html(String::from("Test Details"), html_table(data.tests));

    format!("{}{}", suites_table, tests_summary)
}

fn coverage_html(data: CoverageResult) -> String {
    let summary = format!(
        "<h3>{:.2}% Code Coverage ({}/{})</h3>",
        data.overview.line_coverage, data.overview.lines_covered, data.overview.lines_total,
    );
    let details = html_table(data.files);

    summary_details_html(summary, details)
}

pub(crate) fn gen_summary(test_data: NextestResult, coverage_data: CoverageResult) -> String {
    let header_brief = test_data.overview.clone();

    let test_summary =
        summary_details_html(String::from("<h3>Test Results</h3>"), test_html(test_data));
    let coverage_summary = summary_details_html(
        String::from("<h3>Coverage Reports</h3>"),
        coverage_html(coverage_data),
    );

    format!(
        "<h2>{} tests ran in {} seconds with {} failures</h2>{}{}",
        header_brief.count,
        header_brief.time,
        header_brief.failures,
        test_summary,
        coverage_summary
    )
}
