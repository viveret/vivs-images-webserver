
pub const SQL_CREATE_IMAGE_EXIF: &str = r#"
CREATE TABLE IF NOT EXISTS image_exif (
    image_path TEXT PRIMARY KEY,
    image_taken_at TIMESTAMP NULL,
    
    -- Date and time fields
    date_time TIMESTAMP NULL,
    date_time_original TIMESTAMP NULL,
    date_time_digitized TIMESTAMP NULL,
    gps_date_stamp TEXT NULL,
    gps_time_stamp TEXT NULL,
    sub_sec_time TEXT NULL,
    sub_sec_time_original TEXT NULL,
    sub_sec_time_digitized TEXT NULL,
    offset_time TEXT NULL,
    offset_time_original TEXT NULL,
    offset_time_digitized TEXT NULL,
    
    -- Camera information
    camera_make TEXT NULL,
    camera_model TEXT NULL,
    camera_owner_name TEXT NULL,
    body_serial_number TEXT NULL,
    
    -- Lens information
    lens_model TEXT NULL,
    lens_make TEXT NULL,
    lens_serial_number TEXT NULL,
    lens_specification TEXT NULL,
    focal_length REAL NULL,
    focal_length_in_35mm_film REAL NULL,
    max_aperture_value REAL NULL,
    
    -- Exposure settings
    exposure_time TEXT NULL,
    f_number REAL NULL,
    aperture_value REAL NULL,
    shutter_speed_value REAL NULL,
    exposure_program TEXT NULL,
    exposure_mode TEXT NULL,
    exposure_bias_value REAL NULL,
    exposure_index REAL NULL,
    
    -- ISO and sensitivity
    brightness_value REAL NULL,
    iso_speed INTEGER NULL,
    photographic_sensitivity INTEGER NULL,
    standard_output_sensitivity INTEGER NULL,
    recommended_exposure_index INTEGER NULL,
    iso_speed_latitude_yyy INTEGER NULL,
    iso_speed_latitude_zzz INTEGER NULL,
    sensitivity_type TEXT NULL,
    
    -- Flash information
    flash TEXT NULL,
    flash_energy REAL NULL,
    
    -- Image dimensions and properties
    width INTEGER NULL,
    height INTEGER NULL,
    pixel_x_dimension INTEGER NULL,
    pixel_y_dimension INTEGER NULL,
    orientation INTEGER NULL,
    bits_per_sample TEXT NULL,
    samples_per_pixel INTEGER NULL,
    compression TEXT NULL,
    compressed_bits_per_pixel REAL NULL,
    planar_configuration TEXT NULL,
    photometric_interpretation TEXT NULL,
    
    -- Color and image processing
    color_space TEXT NULL,
    gamma REAL NULL,
    white_balance TEXT NULL,
    white_point TEXT NULL,
    primary_chromaticities TEXT NULL,
    ycbcr_coefficients TEXT NULL,
    ycbcr_positioning TEXT NULL,
    ycbcr_sub_sampling TEXT NULL,
    transfer_function TEXT NULL,
    
    -- Image adjustments
    contrast TEXT NULL,
    saturation TEXT NULL,
    sharpness TEXT NULL,
    gain_control TEXT NULL,
    custom_rendered TEXT NULL,
    
    -- Scene and subject information
    scene_type TEXT NULL,
    scene_capture_type TEXT NULL,
    subject_area TEXT NULL,
    subject_location TEXT NULL,
    subject_distance REAL NULL,
    subject_distance_range TEXT NULL,
    
    -- Camera technical data
    metering_mode TEXT NULL,
    light_source TEXT NULL,
    sensing_method TEXT NULL,
    cfa_pattern TEXT NULL,
    focal_plane_x_resolution REAL NULL,
    focal_plane_y_resolution REAL NULL,
    focal_plane_resolution_unit TEXT NULL,
    
    -- GPS data
    gps_latitude REAL NULL,
    gps_longitude REAL NULL,
    gps_altitude REAL NULL,
    gps_altitude_ref TEXT NULL,
    gps_latitude_ref TEXT NULL,
    gps_longitude_ref TEXT NULL,
    gps_version_id TEXT NULL,
    gps_satellites TEXT NULL,
    gps_status TEXT NULL,
    gps_measure_mode TEXT NULL,
    gps_dop REAL NULL,
    gps_speed REAL NULL,
    gps_speed_ref TEXT NULL,
    gps_track REAL NULL,
    gps_track_ref TEXT NULL,
    gps_img_direction REAL NULL,
    gps_img_direction_ref TEXT NULL,
    gps_map_datum TEXT NULL,
    gps_dest_latitude REAL NULL,
    gps_dest_latitude_ref TEXT NULL,
    gps_dest_longitude REAL NULL,
    gps_dest_longitude_ref TEXT NULL,
    gps_dest_bearing REAL NULL,
    gps_dest_bearing_ref TEXT NULL,
    gps_dest_distance REAL NULL,
    gps_dest_distance_ref TEXT NULL,
    gps_processing_method TEXT NULL,
    gps_area_information TEXT NULL,
    gps_differential TEXT NULL,
    gps_h_positioning_error REAL NULL,
    
    -- Environmental data
    temperature REAL NULL,
    humidity REAL NULL,
    pressure REAL NULL,
    water_depth REAL NULL,
    acceleration REAL NULL,
    camera_elevation_angle REAL NULL,
    
    -- Software and metadata
    software TEXT NULL,
    artist TEXT NULL,
    copyright TEXT NULL,
    image_description TEXT NULL,
    maker_note TEXT NULL,
    user_comment TEXT NULL,
    image_unique_id TEXT NULL,
    related_sound_file TEXT NULL,
    
    -- EXIF version and format
    exif_version TEXT NULL,
    flashpix_version TEXT NULL,
    
    -- File structure
    file_source TEXT NULL,
    jpeg_interchange_format TEXT NULL,
    jpeg_interchange_format_length TEXT NULL,
    strip_offsets TEXT NULL,
    strip_byte_counts TEXT NULL,
    tile_offsets TEXT NULL,
    tile_byte_counts TEXT NULL,
    rows_per_strip INTEGER NULL,
    
    -- Resolution data
    x_resolution REAL NULL,
    y_resolution REAL NULL,
    resolution_unit TEXT NULL,
    
    -- Advanced technical data
    oecf TEXT NULL,
    spectral_sensitivity TEXT NULL,
    spatial_frequency_response TEXT NULL,
    components_configuration TEXT NULL,
    reference_black_white TEXT NULL,
    device_setting_description TEXT NULL,
    
    -- Composite image data
    composite_image TEXT NULL,
    source_image_number_of_composite_image TEXT NULL,
    source_exposure_times_of_composite_image TEXT NULL,
    
    -- Digital zoom
    digital_zoom_ratio REAL NULL,
    
    -- Timestamp for when this record was created/updated
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE UNIQUE INDEX IF NOT EXISTS idx_image_path ON image_exif(image_path);
CREATE INDEX IF NOT EXISTS idx_image_exif_taken_at ON image_exif(image_taken_at);
CREATE INDEX IF NOT EXISTS idx_image_exif_camera_make ON image_exif(camera_make);
CREATE INDEX IF NOT EXISTS idx_image_exif_camera_model ON image_exif(camera_model);
CREATE INDEX IF NOT EXISTS idx_image_exif_iso_speed ON image_exif(iso_speed);
CREATE INDEX IF NOT EXISTS idx_image_exif_focal_length ON image_exif(focal_length);
CREATE INDEX IF NOT EXISTS idx_image_exif_gps_location ON image_exif(gps_latitude, gps_longitude);
CREATE INDEX IF NOT EXISTS idx_image_exif_width_height ON image_exif(width, height);
CREATE INDEX IF NOT EXISTS idx_image_exif_orientation ON image_exif(orientation);


"#;