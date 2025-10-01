use std::collections::HashSet;
use std::f64::consts::PI;

use chrono::{Local, Timelike};

use crate::actions::analysis_task_item_processor::LogProgListenerPair;

pub fn calculate_progress(current: usize, total: usize) -> f32 {
    if total > 0 { (current + 1) as f32 / total as f32 } else { 0.0 }
}


// Generate all unique pairs from a list of image paths
pub fn generate_unique_pairs(image_list: &HashSet<String>, log_prog_listener: Option<LogProgListenerPair>) -> Vec<(String, String)> {
    let total = image_list.len() * (image_list.len() - 1) / 2;
    if let Some(x) = &log_prog_listener { x.1( &format!("generating {} unique pairs", total) ); }
    let mut num_processed = 0;
    let mut pairs = Vec::new();
    for (i, path_a) in image_list.iter().enumerate() {
        for path_b in image_list.iter().skip(i + 1) {
            pairs.push((path_a.clone(), path_b.clone()));
            num_processed += 1;

            if num_processed % 1000 == 0 {
                if let Some(x) = &log_prog_listener { x.0.as_ref()( calculate_progress(num_processed, total) ) }
            }
        }
    }
    pairs
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