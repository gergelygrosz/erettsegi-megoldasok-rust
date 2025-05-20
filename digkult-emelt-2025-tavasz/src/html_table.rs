pub struct HtmlTable {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl HtmlTable {
    pub fn generate_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<table>\n");

        html.push_str("<tr>");
        for header_col in &self.header {
            html.push_str("<th>");
            html.push_str(header_col);
            html.push_str("</th>");
        }
        html.push_str("</th>");

        for row in &self.rows {
            html.push_str("\n");
            html.push_str("<tr>");
            for row_col in row {
                html.push_str("<td>");
                html.push_str(row_col);
                html.push_str("</td>");
            }
            html.push_str("</tr>");
        }

        html.push_str("\n</table>");

        html
    }
}
