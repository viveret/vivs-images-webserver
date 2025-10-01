
use crate::models::query_params::search_params::SearchParams;
use crate::calc::math::calculate_brightness_params;

pub enum ImageQuality {
    XHD, HD, Med, Low
}


// Main query function
pub fn get_image_wallpaper_based_on_brightness_search_params() -> SearchParams {
    let (lower_bound, upper_bound) = calculate_brightness_params();
    let aspect_ratio_min = 1.3;
    let aspect_ratio_max = 3.0;
    let quality = ImageQuality::HD;

    // Get random image matching the brightness range
    let mut params = SearchParams::default();
    params.set_field_value("limit", Some("1".to_string()));
    params.set_field_value("brightness_min", Some(lower_bound.to_string()));
    params.set_field_value("brightness_max", Some(upper_bound.to_string()));
    params.set_field_value("aspect_ratio_min", Some(aspect_ratio_min.to_string()));
    params.set_field_value("aspect_ratio_max", Some(aspect_ratio_max.to_string()));
    // params.set_field_value("image_path", Some("Wallpapers/".to_string()));

    params
}