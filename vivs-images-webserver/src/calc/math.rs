pub fn calculate_progress(current: usize, total: usize) -> f32 {
    if total > 0 { (current + 1) as f32 / total as f32 } else { 0.0 }
}