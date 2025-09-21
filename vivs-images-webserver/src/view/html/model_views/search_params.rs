
pub fn use_simple_view_checkbox(checked: bool) -> String {
    if checked {
        r#"<label><input type="checkbox" name="use_simple_view" value="true" checked> Use Simple View</label>"#.to_string()
    } else {
        r#"<label><input type="checkbox" name="use_simple_view" value="true"> Use Simple View</label>"#.to_string()
    }
}

pub fn query_string_input(value: &str) -> String {
    format!(r#"<input type="text" name="query" placeholder="Search..." value="{}">"#, value)
}