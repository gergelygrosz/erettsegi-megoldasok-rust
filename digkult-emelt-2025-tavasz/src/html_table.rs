pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<i32>>,
}

impl Table {
    pub fn generate_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<table>\n");

        html.push_str("<tr>");
        for header in &self.headers {
            html.push_str("<th>");
            html.push_str(header);
            html.push_str("</th>");
        }
        html.push_str("</tr>");

        for row in &self.body {
            html.push_str("\n");
            html.push_str("<tr>");
            for column in row {
                html.push_str("<td>");
                html.push_str(column.to_string().as_str());
                html.push_str("</td>");
            }
            html.push_str("</tr>");
        }

        html.push_str("\n</table>");

        html
    }

    pub fn generate_string(&self) -> String {
        let mut output = String::new();

        for header in &self.headers {
            output.push_str(header);
            output.push_str("\t");
        }
        output.push_str("\n");

        for row in &self.body {
            for column in row {
                output.push_str(column.to_string().as_str());
                output.push_str("\t\t");
            }
            output.push_str("\n");
        }

        output
    }
}
