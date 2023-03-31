use super::parse_data::{CoverageFileAnalysis, CoverageResult, NextestResult, NextestTest};

pub trait HTMLTable {
    fn to_th(&self) -> String;
    fn to_td(&self) -> String;
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
    let suites_summary = format!("<h3>Suites ({}) Details</h3>", data.suites.len());
    let mut suites_details = String::new();

    for suite in data.suites {
        let tests: Vec<NextestTest> =
            data.tests.iter().filter(|x| x.suite == suite.name).cloned().collect();
        let inner_summary = format!("<h4>{} ({})</h4>", suite.name, tests.len());
        let inner_details = html_table(tests);

        suites_details.push_str(&summary_details_html(inner_summary, inner_details))
    }

    format!(
        "## {} tests ran in {}s with {} failures\n\n{}",
        data.overview.count,
        data.overview.time,
        data.overview.failures,
        summary_details_html(suites_summary, suites_details)
    )
}

fn coverage_html(data: CoverageResult) -> String {
    let summary = format!(
        "<h3>{:.2}% Code Coverage ({}/{})</h3>",
        data.overview.line_coverage, data.overview.lines_covered, data.overview.lines_total,
    );
    let details = html_table(data.files);

    summary_details_html(summary, details)
}

pub fn gen_summary(test_data: NextestResult, coverage_data: CoverageResult) -> String {
    format!("{}\n\n{}", test_html(test_data), coverage_html(coverage_data))
}
