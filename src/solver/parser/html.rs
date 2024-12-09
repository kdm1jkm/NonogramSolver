use super::{SolverParseResult, SolverParser};
use crate::board::Vec2;
use regex::Regex;

pub struct HtmlTableSolverParser<'a> {
    html_table: &'a str,
}

impl<'a> HtmlTableSolverParser<'a> {
    pub fn new(html_table: &'a str) -> Self {
        Self { html_table }
    }
}

impl SolverParser for HtmlTableSolverParser<'_> {
    fn parse(&self) -> Result<SolverParseResult, String> {
        let column_td_re =
            Regex::new(r#"<td data-row="-1" data-col="\d+"[^>]*>(.*?)</td>"#).unwrap();
        let number_re = Regex::new(r"<span>(\d+)</span>").unwrap();

        // 열 힌트 추출
        let mut column_hints = Vec::new();
        for td_caps in column_td_re.captures_iter(self.html_table) {
            let td_content = td_caps.get(1).unwrap().as_str();
            let numbers: Vec<usize> = number_re
                .captures_iter(td_content)
                .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
                .collect();
            column_hints.push(numbers);
        }

        // 행 힌트 추출
        let tbody_re = Regex::new(r"<tbody>(.*?)</tbody>").unwrap();
        let first_td_div_re =
            Regex::new(r"<tr[^>]*?>.*?<td[^>]*?><div>((?:<span>\d+<\/span>)+)</div></td>").unwrap();

        let mut row_hints = Vec::new();
        if let Some(tbody_caps) = tbody_re.captures(self.html_table) {
            let tbody_content = tbody_caps.get(1).unwrap().as_str();

            for td_caps in first_td_div_re.captures_iter(tbody_content) {
                let div_content = td_caps.get(1).unwrap().as_str();
                let numbers: Vec<usize> = number_re
                    .captures_iter(div_content)
                    .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
                    .collect();
                row_hints.push(numbers);
            }
        }

        println!("row_hints: {:?}", row_hints);
        println!("column_hints: {:?}", column_hints);
        // print length of row_hints and column_hints
        println!("row_hints.len(): {}", row_hints.len());
        println!("column_hints.len(): {}", column_hints.len());

        if !row_hints.is_empty() && !column_hints.is_empty() {
            Ok(SolverParseResult {
                board_size: Vec2 {
                    row: row_hints.len(),
                    column: column_hints.len(),
                },
                row_hints,
                column_hints,
            })
        } else {
            Err("Failed to parse HTML table".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        display::ConsoleDisplay,
        solver::parser::{HtmlTableSolverParser, SolverParser},
    };

    #[test]
    fn test_create_solver_from_html_table() {
        let html_table = include_str!("../../../sample/table/data2.txt");
        let result = HtmlTableSolverParser::new(html_table)
            .create_solver(Box::new(ConsoleDisplay::new_with_default()));
        assert!(
            result.is_ok(),
            "Failed to create solver: {:?}",
            result.err()
        );

        let result = result.unwrap().solve();
        assert!(result.is_ok(), "Failed to solve: {:?}", result.err());
    }
}
