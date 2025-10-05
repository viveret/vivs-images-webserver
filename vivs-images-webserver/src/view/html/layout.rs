
pub fn layout_view(title: Option<&str>, inner_content: &str) -> String {
    let title_prefix = "Viv's Image Explorer";
    let title = match title {
        Some(t) => format!("{} - {}", t, title_prefix),
        None => title_prefix.to_string(),
    };

    format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>{}</title>
        <link href="/style.css" rel="stylesheet">
    </head>
    <body>
        <h1>{}</h1>

        <div class="nav">
            <a href="/">Home</a> |
            <a href="/actions">Actions</a> |
            <a href="/search">Search</a> |
            <a href="/browse/filesystem">Browse Filesystem</a> |
            <a href="/browse/tags">Browse Tags</a> |
            <a href="/browse/by-property">Browse By Property</a>
        </div>
        {}
    </body>
    </html>
    "#, title, title, inner_content)
}
