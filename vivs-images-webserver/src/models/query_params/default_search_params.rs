use std::f64::consts::PI;

use chrono::{Local, Timelike};

use crate::models::query_params::search_params::SearchParams;



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
pub fn get_image_wallpaper_based_on_brightness_search_params() -> SearchParams {
    let (lower_bound, upper_bound) = calculate_brightness_params();
    
    // Get random image matching the brightness range
    let mut params = SearchParams::default();
    params.set_field_value("limit", Some("1".to_string()));
    params.set_field_value("brightness_min", Some(lower_bound.to_string()));
    params.set_field_value("brightness_max", Some(upper_bound.to_string()));
    params.set_field_value("query", Some("Wallpapers/".to_string()));

    params
}