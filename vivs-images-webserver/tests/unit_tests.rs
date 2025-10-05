extern crate image_exif_explorer;

mod tests {
    use super::*;
    use image_exif_explorer::core::data_context::WebServerActionDataContext;
    use image_exif_explorer::api::api_get_wallpaper_image_path::api_get_wallpaper_image_path_inner;
    
    #[tokio::test]
    async fn test_get_wallpaper_api() {
        let pool = WebServerActionDataContext::open().await.expect("data");
        let result = api_get_wallpaper_image_path_inner(pool).await;
        match result {
            Ok(w) => {
                if let Some(w) = w {
                    assert!(!w.is_empty());
                    assert!(w.len() > 10);
                } else {
                    panic!("wallpaper was none")
                }
            },
            Err(e) => {
                panic!("could not get wallpaper: {}", e)
            },
        }
    }
}