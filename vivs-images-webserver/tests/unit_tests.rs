extern crate image_exif_explorer;

mod tests {
    use super::*;
    use image_exif_explorer::models::config::paths::DB_FILE;
    use image_exif_explorer::api::api_get_wallpaper_image_path::api_get_wallpaper_image_path_inner;
    use sqlx::SqlitePool;
    
    #[tokio::test]
    async fn test_get_wallpaper_api() {
        let pool = SqlitePool::connect(&format!("sqlite:{}", DB_FILE)).await;
        match pool {
            Ok(pool) => {
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
            Err(e) => panic!("could not get sql pool: {}", e)
        }
    }
}