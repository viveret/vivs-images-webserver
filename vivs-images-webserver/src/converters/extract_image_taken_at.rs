use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use exif::{In, Tag};
use regex::Regex;

pub fn extract_image_taken_at(image_path: &str) -> actix_web::Result<Option<String>> {
    let mut taken_at: Option<String> = None;

    // try exif first
    taken_at = extract_image_date_from_exif(image_path)?;
    // println!("extract_image_taken_at exif_string = {:?}", taken_at);
    if taken_at.is_some() {
        return Ok(taken_at)
    }

    // try the file name
    taken_at = extract_image_date_from_file_name(image_path);
    // println!("extract_image_taken_at file_name = {:?}", taken_at);    
    if taken_at.is_some() {
        return Ok(taken_at)
    }

    // final attempt is to get file creation time
    // println!("extract_image_taken_at file_creation_time = {:?}", taken_at);    

    Ok(taken_at)
}

fn extract_image_date_from_exif(image_path: &str) -> actix_web::Result<Option<String>> {
    let file = std::fs::File::open(image_path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?;
    
    // Try various EXIF date fields (different cameras use different fields)
    for tag in [Tag::DateTimeOriginal, Tag::DateTimeDigitized, Tag::DateTime] {
        if let Some(x) = exif.get_field(tag, In::PRIMARY) {
            return Ok(Some(x.display_value().to_string()))
        }
    }

    Ok(None)
}

// Function to extract date from filename (common camera naming patterns)
// Common camera filename patterns:
// IMG_YYYYMMDD_HHMMSS.jpg
// DSC_YYYYMMDD_HHMMSS.jpg
// YYYYMMDD_HHMMSS.jpg
// IMG-YYYYMMDD-WA0000.jpg (WhatsApp)
fn extract_image_date_from_file_name(image_path: &str) -> Option<String> {
    let filename = std::path::Path::new(image_path)
        .file_name().unwrap()
        .to_str().unwrap()
        .to_lowercase();
    
    let patterns = [
        // Pattern 1: IMG_YYYYMMDD_HHMMSS or DSC_YYYYMMDD_HHMMSS
        r"(?:img|dsc)_(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})",
        // Pattern 2: YYYYMMDD_HHMMSS (standalone)
        r"(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})",
        // Pattern 3: IMG-YYYYMMDD-WA0000 (WhatsApp)
        r"(?:img|dsc)-(\d{4})(\d{2})(\d{2})-wa\d+",
        // Pattern 4: YYYYMMDD_HHMMSS with various separators
        r"(\d{4})(\d{2})(\d{2})[-_](\d{2})(\d{2})(\d{2})",
    ];

    for pattern in patterns.iter() {
        if let Some(dt) = extract_datetime_from_pattern(&filename, pattern) {
            return Some(dt.to_string());
        }
    }
    
    None
}


/// Extract datetime using a specific regex pattern
fn extract_datetime_from_pattern(text: &str, pattern: &str) -> Option<DateTime<Utc>> {
    let re = Regex::new(pattern).ok()?;
    let captures = re.captures(text)?;
    
    if captures.len() >= 7 { // 6 groups + full match
        let year = captures.get(1)?.as_str().parse::<i32>().ok()?;
        let month = captures.get(2)?.as_str().parse::<u32>().ok()?;
        let day = captures.get(3)?.as_str().parse::<u32>().ok()?;
        let hour = captures.get(4)?.as_str().parse::<u32>().ok()?;
        let minute = captures.get(5)?.as_str().parse::<u32>().ok()?;
        let second = captures.get(6)?.as_str().parse::<u32>().ok()?;
        
        // Try to create a NaiveDateTime first
        if let Some(naive_dt) = NaiveDateTime::from_timestamp_opt(
            create_timestamp(year, month, day, hour, minute, second)?, 
            0
        ) {
            return Some(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
        }
    }
    
    None
}


/// Helper function to create timestamp from components
fn create_timestamp(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Option<i64> {
    // Basic validation
    if month < 1 || month > 12 || day < 1 || day > 31 || hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    
    // Use chrono to create proper timestamp
    NaiveDateTime::from_timestamp_opt(0, 0)
        .and_then(|base| {
            base.date()
                .with_year(year)
                .and_then(|d| d.with_month(month))
                .and_then(|d| d.with_day(day))
                .and_then(|d| d.and_hms_opt(hour, minute, second))
        })
        .map(|dt| dt.timestamp())
}
