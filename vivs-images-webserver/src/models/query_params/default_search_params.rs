
use crate::models::image_aspect_ratio::ImageQuality;
use crate::models::query_params::search_params::SearchParams;
use crate::calc::math::calculate_brightness_params;



// Main query function
pub fn get_image_wallpaper_based_on_brightness_search_params() -> SearchParams {
    let (lower_bound, upper_bound) = calculate_brightness_params();
    let aspect_ratio_min = 1.3;
    let aspect_ratio_max = 3.0;
    let quality_min = ImageQuality::HD;

    // Get random image matching the brightness range
    let mut params = SearchParams::default();
    params.set_field_value("limit", Some("1".to_string())).expect("could not set limit");
    params.set_field_value("brightness_min", Some(lower_bound.to_string())).expect("could not set brightness_min");
    params.set_field_value("brightness_max", Some(upper_bound.to_string())).expect("could not set brightness_max");
    params.set_field_value("aspect_ratio_min", Some(aspect_ratio_min.to_string())).expect("could not set aspect_ratio_min");
    params.set_field_value("aspect_ratio_max", Some(aspect_ratio_max.to_string())).expect("could not set aspect_ratio_max");
    params.set_field_value("quality_min", Some((quality_min as u8).to_string())).expect("could not set quality_min");

    params
}