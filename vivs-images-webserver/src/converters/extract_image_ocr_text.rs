use std::{io::{Error, ErrorKind}, process::Command};


pub fn extract_image_ocr_text(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Run Tesseract OCR compare command
    let output = Command::new("tesseract")
        .args([image_path, "-"])
        .output()?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        Ok(text.to_string())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(Box::new(Error::new(ErrorKind::Other, format!("Tesseract error: {}", error_msg))));
    }
}