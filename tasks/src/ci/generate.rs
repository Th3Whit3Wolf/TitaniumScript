use super::parse_data::{
    CoverageFileAnalysis, CoverageResult, NextestResult, NextestSuite, NextestTest,
};

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
            if self.failure { "🔴" } else { "🟢" }
        )
    }
}

impl HTMLTable for CoverageFileAnalysis {
    fn to_th(&self) -> String {
        "<tr><th>File</th><th>Coverage</th></tr>".to_string()
    }
    fn to_td(&self) -> String {
        format!("<tr><td width=\"75%\"><a href=\"https://github.com/Th3Whit3Wolf/TitaniumScript/blob/main{}\">{}</a></td><td>{:.2}%</td></tr>", self.path, self.path, self.line_coverage,)
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
    format!(
        "<h3>{:.2}% Code Coverage ({}/{})</h3>{}",
        data.overview.line_coverage,
        data.overview.lines_covered,
        data.overview.lines_total,
        html_table(data.files)
    )
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
