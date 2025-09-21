

// Generate filter form for a specific category
pub fn generate_category_filter_form(category: &str) -> String {
    let display_name = match category {
        "camera_model" => "Camera Model",
        "camera_make" => "Camera Make",
        "lens_model" => "Lens Model",
        "iso_speed" => "ISO Speed",
        "focal_length" => "Focal Length",
        _ => category,
    };

    format!(r#"
    <div class="search-form">
        <h3>Filter by {}</h3>
        <form action="/categories/{}" method="get">
            <input type="text" name="value" placeholder="{}..." value="{{value}}">
            <input type="submit" value="Filter">
        </form>
    </div>
    "#, display_name, category, display_name)
}