use std::f64::consts::PI;
use chrono::{Local, Timelike};

// Generate brightness SQL conditional part of the query
pub fn generate_brightness_conditional_query_part(lower_bound: f64, upper_bound: f64) -> String {
    format!("brightness BETWEEN {} AND {} AND brightness IS NOT NULL", lower_bound, upper_bound)
}

// Generate brightness SQL query
pub fn generate_brightness_query(lower_bound: f64, upper_bound: f64) -> String {
    let inner_sql = generate_brightness_conditional_query_part(lower_bound, upper_bound);
    let sql_query = format!(
        "SELECT image_path FROM image_brightness WHERE {} ORDER BY RANDOM() LIMIT 1;",
        inner_sql
    );

    sql_query
}

// Calculate brightness parameters based on current time
pub fn calculate_brightness_params() -> (f64, f64) {
    let now = Local::now();
    let hour = now.hour() as f64;
    let minute = now.minute() as f64;
    let time_decimal = hour + minute / 60.0;

    // Smooth brightness transition throughout the day
    // Brightness follows a sine wave pattern: lowest at night, highest at noon
    let time_rad = time_decimal * 2.0 * PI / 24.0;
    let target_brightness = 0.5 + 0.4 * time_rad.sin();

    // Adjust range based on time (wider range during transitions)
    let brightness_range = if (5.0..7.0).contains(&hour) {
        0.2  // Wider range during dawn
    } else if (17.0..19.0).contains(&hour) {
        0.2  // Wider range during dusk
    } else {
        0.15 // Narrower range otherwise
    };

    // Calculate bounds
    let mut lower_bound = target_brightness - brightness_range;
    let mut upper_bound = target_brightness + brightness_range;

    // Ensure bounds are within 0-1
    lower_bound = lower_bound.clamp(0.0, 1.0);
    upper_bound = upper_bound.clamp(0.0, 1.0);

    (lower_bound, upper_bound)
}

// Main query function
pub fn get_image_brightness_query() -> String {
    let (lower_bound, upper_bound) = calculate_brightness_params();
    
    // Get random image matching the brightness range
    let random_image = generate_brightness_query(lower_bound, upper_bound);
    
    random_image
}