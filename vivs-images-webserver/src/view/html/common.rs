

// Helper function to generate image HTML with common styling
pub fn image_html(image_path: &str, max_width: Option<u32>) -> String {
    let width_style = max_width.map(|w| format!("max-width: {}px;", w)).unwrap_or_default();
    format!(r#"<img src="/img?path={}" alt="{}" onerror="this.style.display='none'" style="{}">"#,
        image_path, image_path, width_style)
}

pub fn link_html(href: String, inner_content: &str) -> String {
    format!(r#"<a href="{}">{}</a>"#, href, inner_content)
}

pub fn encode_string(input: &str) -> String {
    let mut output = String::new();
    url_escape::encode_path_to_string(input, &mut output);
    output
}

// Helper function to create HTML table with headers
pub fn create_html_table(title: &str, headers: &Vec<String>, rows_html: &str) -> String {
    let mut table_headers = String::new();

    for header in headers {
        table_headers.push_str(&format!("<th>{}</th>", header));
    }

    let full_html = format!(r#"
    <h2>{}</h2>
    <table>
        <tr>{}</tr>
        {}
    </table>
    "#, title, table_headers, rows_html);

    full_html
}