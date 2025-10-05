use convert_case::Casing;
use sqlx::Row;

use crate::models::image::ImageFieldMeta;

#[derive(Clone, Debug)]
pub struct ImageIptc {
    pub image_path: String,

    pub model_version: Option<String>,
    pub date_sent: Option<String>,
    pub time_sent: Option<String>,
    pub coded_character_set: Option<String>,
    pub record_version: Option<String>,
    pub object_type_reference: Option<String>,
    pub object_attribute_reference: Option<String>,
    pub object_name: Option<String>,
    pub edit_status: Option<String>,
    pub editorial_update: Option<String>,
    pub urgency: Option<String>,
    pub subject_reference: Option<String>,
    pub category: Option<String>,
    pub supplemental_categories: Option<String>,
    pub fixture_id: Option<String>,
    pub keywords: Option<String>,
    pub content_location_code: Option<String>,
    pub content_location_name: Option<String>,
    pub release_date: Option<String>,
    pub release_time: Option<String>,
    pub expiration_date: Option<String>,
    pub expiration_time: Option<String>,
    pub special_instructions: Option<String>,
    pub action_advised: Option<String>,
    pub reference_service: Option<String>,
    pub reference_date: Option<String>,
    pub reference_number: Option<String>,
    pub date_created: Option<String>,
    pub time_created: Option<String>,
    pub digital_date_created: Option<String>,
    pub digital_time_created: Option<String>,
    pub originating_program: Option<String>,
    pub program_version: Option<String>,
    pub object_cycle: Option<String>,
    pub by_line: Option<String>,
    pub by_line_title: Option<String>,
    pub city: Option<String>,
    pub sub_location: Option<String>,
    pub province_or_state: Option<String>,
    pub country_or_primary_location_code: Option<String>,
    pub country_or_primary_location_name: Option<String>,
    pub original_transmission_reference: Option<String>,
    pub headline: Option<String>,
    pub credit: Option<String>,
    pub source: Option<String>,
    pub copyright_notice: Option<String>,
    pub contact: Option<String>,
    pub caption: Option<String>,
    pub local_caption: Option<String>,
    pub caption_writer: Option<String>,
    pub rasterized_caption: Option<String>,
    pub image_type: Option<String>,
    pub image_orientation: Option<String>,
    pub language_identifier: Option<String>,
    pub audio_type: Option<String>,
    pub audio_sampling_rate: Option<String>,
    pub audio_sampling_resolution: Option<String>,
    pub audio_duration: Option<String>,
    pub audio_outcue: Option<String>,
    pub job_id: Option<String>,
    pub master_document_id: Option<String>,
    pub short_document_id: Option<String>,
    pub unique_document_id: Option<String>,
    pub owner_id: Option<String>,
}

pub const IMAGE_IPTC_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_iptc"},
    {"name": "model_version", "label": "Model Version", "description": "IPTC model version", "field_type": "string", "example": "4", "category": "iptc", "table_name": "image_iptc"},
    {"name": "date_sent", "label": "Date Sent", "description": "Date the service sent the material", "field_type": "string", "example": "20230815", "category": "iptc", "table_name": "image_iptc"},
    {"name": "time_sent", "label": "Time Sent", "description": "Time the service sent the material", "field_type": "string", "example": "143500", "category": "iptc", "table_name": "image_iptc"},
    {"name": "coded_character_set", "label": "Coded Character Set", "description": "Character set used", "field_type": "string", "example": "UTF-8", "category": "iptc", "table_name": "image_iptc"},
    {"name": "record_version", "label": "Record Version", "description": "IPTC record version", "field_type": "string", "example": "4", "category": "iptc", "table_name": "image_iptc"},
    {"name": "object_type_reference", "label": "Object Type Reference", "description": "Type of object", "field_type": "string", "example": "News", "category": "iptc", "table_name": "image_iptc"},
    {"name": "object_attribute_reference", "label": "Object Attribute Reference", "description": "Attribute of the object", "field_type": "string", "example": "Feature", "category": "iptc", "table_name": "image_iptc"},
    {"name": "object_name", "label": "Object Name", "description": "Short verbal reference for the object", "field_type": "string", "example": "Summer Festival", "category": "iptc", "table_name": "image_iptc"},
    {"name": "edit_status", "label": "Edit Status", "description": "Status of the object data", "field_type": "string", "example": "Released", "category": "iptc", "table_name": "image_iptc"},
    {"name": "editorial_update", "label": "Editorial Update", "description": "Indicates the content has been updated", "field_type": "string", "example": "Additional language", "category": "iptc", "table_name": "image_iptc"},
    {"name": "urgency", "label": "Urgency", "description": "Editorial urgency of content", "field_type": "string", "example": "1", "category": "iptc", "table_name": "image_iptc"},
    {"name": "subject_reference", "label": "Subject Reference", "description": "Subject of the content", "field_type": "string", "example": "SOC, CULTURE", "category": "iptc", "table_name": "image_iptc"},
    {"name": "category", "label": "Category", "description": "Subject of the object data", "field_type": "string", "example": "NEW", "category": "iptc", "table_name": "image_iptc"},
    {"name": "supplemental_categories", "label": "Supplemental Categories", "description": "Supplemental categories", "field_type": "string", "example": "SPT, FEA", "category": "iptc", "table_name": "image_iptc"},
    {"name": "fixture_id", "label": "Fixture ID", "description": "Fixture identifier", "field_type": "string", "example": "FIX123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "keywords", "label": "Keywords", "description": "Keywords for the content", "field_type": "string", "example": "festival, summer, music", "category": "iptc", "table_name": "image_iptc"},
    {"name": "content_location_code", "label": "Content Location Code", "description": "Code of the country/primary location", "field_type": "string", "example": "USA", "category": "iptc", "table_name": "image_iptc"},
    {"name": "content_location_name", "label": "Content Location Name", "description": "Name of the country/primary location", "field_type": "string", "example": "United States", "category": "iptc", "table_name": "image_iptc"},
    {"name": "release_date", "label": "Release Date", "description": "Date the content is released", "field_type": "string", "example": "20230816", "category": "iptc", "table_name": "image_iptc"},
    {"name": "release_time", "label": "Release Time", "description": "Time the content is released", "field_type": "string", "example": "090000", "category": "iptc", "table_name": "image_iptc"},
    {"name": "expiration_date", "label": "Expiration Date", "description": "Date the content expires", "field_type": "string", "example": "20240816", "category": "iptc", "table_name": "image_iptc"},
    {"name": "expiration_time", "label": "Expiration Time", "description": "Time the content expires", "field_type": "string", "example": "235959", "category": "iptc", "table_name": "image_iptc"},
    {"name": "special_instructions", "label": "Special Instructions", "description": "Editorial instructions", "field_type": "string", "example": "For online use only", "category": "iptc", "table_name": "image_iptc"},
    {"name": "action_advised", "label": "Action Advised", "description": "Action provided to the object", "field_type": "string", "example": "KILL", "category": "iptc", "table_name": "image_iptc"},
    {"name": "reference_service", "label": "Reference Service", "description": "Service identifier", "field_type": "string", "example": "REF123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "reference_date", "label": "Reference Date", "description": "Date of a prior envelope", "field_type": "string", "example": "20230801", "category": "iptc", "table_name": "image_iptc"},
    {"name": "reference_number", "label": "Reference Number", "description": "Number of a prior envelope", "field_type": "string", "example": "456", "category": "iptc", "table_name": "image_iptc"},
    {"name": "date_created", "label": "Date Created", "description": "Date the intellectual content was created", "field_type": "string", "example": "20230815", "category": "iptc", "table_name": "image_iptc"},
    {"name": "time_created", "label": "Time Created", "description": "Time the intellectual content was created", "field_type": "string", "example": "140000", "category": "iptc", "table_name": "image_iptc"},
    {"name": "digital_date_created", "label": "Digital Date Created", "description": "Date the digital representation was created", "field_type": "string", "example": "20230815", "category": "iptc", "table_name": "image_iptc"},
    {"name": "digital_time_created", "label": "Digital Time Created", "description": "Time the digital representation was created", "field_type": "string", "example": "141500", "category": "iptc", "table_name": "image_iptc"},
    {"name": "originating_program", "label": "Originating Program", "description": "Program used to create the object", "field_type": "string", "example": "Adobe Photoshop", "category": "iptc", "table_name": "image_iptc"},
    {"name": "program_version", "label": "Program Version", "description": "Version of the program", "field_type": "string", "example": "24.0", "category": "iptc", "table_name": "image_iptc"},
    {"name": "object_cycle", "label": "Object Cycle", "description": "Morning, evening, or both", "field_type": "string", "example": "a", "category": "iptc", "table_name": "image_iptc"},
    {"name": "by_line", "label": "By Line", "description": "Name of the creator", "field_type": "string", "example": "John Smith", "category": "iptc", "table_name": "image_iptc"},
    {"name": "by_line_title", "label": "By Line Title", "description": "Title of the creator", "field_type": "string", "example": "Photographer", "category": "iptc", "table_name": "image_iptc"},
    {"name": "city", "label": "City", "description": "City of the origin", "field_type": "string", "example": "New York", "category": "iptc", "table_name": "image_iptc"},
    {"name": "sub_location", "label": "Sub Location", "description": "Location within a city", "field_type": "string", "example": "Central Park", "category": "iptc", "table_name": "image_iptc"},
    {"name": "province_or_state", "label": "Province or State", "description": "Province/State of origin", "field_type": "string", "example": "NY", "category": "iptc", "table_name": "image_iptc"},
    {"name": "country_or_primary_location_code", "label": "Country or Primary Location Code", "description": "Country code of origin", "field_type": "string", "example": "US", "category": "iptc", "table_name": "image_iptc"},
    {"name": "country_or_primary_location_name", "label": "Country or Primary Location Name", "description": "Country name of origin", "field_type": "string", "example": "United States", "category": "iptc", "table_name": "image_iptc"},
    {"name": "original_transmission_reference", "label": "Original Transmission Reference", "description": "Original transmission reference", "field_type": "string", "example": "OTR123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "headline", "label": "Headline", "description": "A publishable entry", "field_type": "string", "example": "Summer Music Festival", "category": "iptc", "table_name": "image_iptc"},
    {"name": "credit", "label": "Credit", "description": "Provider of the object", "field_type": "string", "example": "Getty Images", "category": "iptc", "table_name": "image_iptc"},
    {"name": "source", "label": "Source", "description": "Original owner of the intellectual content", "field_type": "string", "example": "Reuters", "category": "iptc", "table_name": "image_iptc"},
    {"name": "copyright_notice", "label": "Copyright Notice", "description": "Copyright notice", "field_type": "string", "example": "Copyright 2023 John Smith", "category": "iptc", "table_name": "image_iptc"},
    {"name": "contact", "label": "Contact", "description": "Contact information", "field_type": "string", "example": "info@example.com", "category": "iptc", "table_name": "image_iptc"},
    {"name": "caption", "label": "Caption", "description": "Description of the object", "field_type": "string", "example": "People enjoying summer festival", "category": "iptc", "table_name": "image_iptc"},
    {"name": "local_caption", "label": "Local Caption", "description": "Caption in local language", "field_type": "string", "example": "Festival d'été", "category": "iptc", "table_name": "image_iptc"},
    {"name": "caption_writer", "label": "Caption Writer", "description": "Writer of the caption", "field_type": "string", "example": "Jane Doe", "category": "iptc", "table_name": "image_iptc"},
    {"name": "rasterized_caption", "label": "Rasterized Caption", "description": "Caption stored as raster data", "field_type": "string", "example": "Raster data", "category": "iptc", "table_name": "image_iptc"},
    {"name": "image_type", "label": "Image Type", "description": "Color model of the image", "field_type": "string", "example": "RGB", "category": "iptc", "table_name": "image_iptc"},
    {"name": "image_orientation", "label": "Image Orientation", "description": "Orientation of the image", "field_type": "string", "example": "Landscape", "category": "iptc", "table_name": "image_iptc"},
    {"name": "language_identifier", "label": "Language Identifier", "description": "Language of the object", "field_type": "string", "example": "en", "category": "iptc", "table_name": "image_iptc"},
    {"name": "audio_type", "label": "Audio Type", "description": "Type of audio content", "field_type": "string", "example": "Mono", "category": "iptc", "table_name": "image_iptc"},
    {"name": "audio_sampling_rate", "label": "Audio Sampling Rate", "description": "Audio sampling rate", "field_type": "string", "example": "44100", "category": "iptc", "table_name": "image_iptc"},
    {"name": "audio_sampling_resolution", "label": "Audio Sampling Resolution", "description": "Audio sampling resolution", "field_type": "string", "example": "16", "category": "iptc", "table_name": "image_iptc"},
    {"name": "audio_duration", "label": "Audio Duration", "description": "Duration of audio content", "field_type": "string", "example": "00:03:45", "category": "iptc", "table_name": "image_iptc"},
    {"name": "audio_outcue", "label": "Audio Outcue", "description": "Content at the end of audio", "field_type": "string", "example": "fade out", "category": "iptc", "table_name": "image_iptc"},
    {"name": "job_id", "label": "Job ID", "description": "Job identifier", "field_type": "string", "example": "JOB123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "master_document_id", "label": "Master Document ID", "description": "Master document identifier", "field_type": "string", "example": "MD123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "short_document_id", "label": "Short Document ID", "description": "Short document identifier", "field_type": "string", "example": "SD123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "unique_document_id", "label": "Unique Document ID", "description": "Unique document identifier", "field_type": "string", "example": "UD123", "category": "iptc", "table_name": "image_iptc"},
    {"name": "owner_id", "label": "Owner ID", "description": "Owner identifier", "field_type": "string", "example": "OWNER123", "category": "iptc", "table_name": "image_iptc"}
]
"#;

impl ImageIptc {
    pub fn new(r: &sqlx::sqlite::SqliteRow) -> Self {
        Self {
            image_path: r.try_get("image_path").ok().unwrap_or_default(),
            model_version: r.try_get("model_version").ok(),
            date_sent: r.try_get("date_sent").ok(),
            time_sent: r.try_get("time_sent").ok(),
            coded_character_set: r.try_get("coded_character_set").ok(),
            record_version: r.try_get("record_version").ok(),
            object_type_reference: r.try_get("object_type_reference").ok(),
            object_attribute_reference: r.try_get("object_attribute_reference").ok(),
            object_name: r.try_get("object_name").ok(),
            edit_status: r.try_get("edit_status").ok(),
            editorial_update: r.try_get("editorial_update").ok(),
            urgency: r.try_get("urgency").ok(),
            subject_reference: r.try_get("subject_reference").ok(),
            category: r.try_get("category").ok(),
            supplemental_categories: r.try_get("supplemental_categories").ok(),
            fixture_id: r.try_get("fixture_id").ok(),
            keywords: r.try_get("keywords").ok(),
            content_location_code: r.try_get("content_location_code").ok(),
            content_location_name: r.try_get("content_location_name").ok(),
            release_date: r.try_get("release_date").ok(),
            release_time: r.try_get("release_time").ok(),
            expiration_date: r.try_get("expiration_date").ok(),
            expiration_time: r.try_get("expiration_time").ok(),
            special_instructions: r.try_get("special_instructions").ok(),
            action_advised: r.try_get("action_advised").ok(),
            reference_service: r.try_get("reference_service").ok(),
            reference_date: r.try_get("reference_date").ok(),
            reference_number: r.try_get("reference_number").ok(),
            date_created: r.try_get("date_created").ok(),
            time_created: r.try_get("time_created").ok(),
            digital_date_created: r.try_get("digital_date_created").ok(),
            digital_time_created: r.try_get("digital_time_created").ok(),
            originating_program: r.try_get("originating_program").ok(),
            program_version: r.try_get("program_version").ok(),
            object_cycle: r.try_get("object_cycle").ok(),
            by_line: r.try_get("by_line").ok(),
            by_line_title: r.try_get("by_line_title").ok(),
            city: r.try_get("city").ok(),
            sub_location: r.try_get("sub_location").ok(),
            province_or_state: r.try_get("province_or_state").ok(),
            country_or_primary_location_code: r.try_get("country_or_primary_location_code").ok(),
            country_or_primary_location_name: r.try_get("country_or_primary_location_name").ok(),
            original_transmission_reference: r.try_get("original_transmission_reference").ok(),
            headline: r.try_get("headline").ok(),
            credit: r.try_get("credit").ok(),
            source: r.try_get("source").ok(),
            copyright_notice: r.try_get("copyright_notice").ok(),
            contact: r.try_get("contact").ok(),
            caption: r.try_get("caption").ok(),
            local_caption: r.try_get("local_caption").ok(),
            caption_writer: r.try_get("caption_writer").ok(),
            rasterized_caption: r.try_get("rasterized_caption").ok(),
            image_type: r.try_get("image_type").ok(),
            image_orientation: r.try_get("image_orientation").ok(),
            language_identifier: r.try_get("language_identifier").ok(),
            audio_type: r.try_get("audio_type").ok(),
            audio_sampling_rate: r.try_get("audio_sampling_rate").ok(),
            audio_sampling_resolution: r.try_get("audio_sampling_resolution").ok(),
            audio_duration: r.try_get("audio_duration").ok(),
            audio_outcue: r.try_get("audio_outcue").ok(),
            job_id: r.try_get("job_id").ok(),
            master_document_id: r.try_get("master_document_id").ok(),
            short_document_id: r.try_get("short_document_id").ok(),
            unique_document_id: r.try_get("unique_document_id").ok(),
            owner_id: r.try_get("owner_id").ok(),
        }
    }

    pub fn default(image_path: &str) -> Self {
        Self {
            image_path: image_path.to_string(),
            model_version: None,
            date_sent: None,
            time_sent: None,
            coded_character_set: None,
            record_version: None,
            object_type_reference: None,
            object_attribute_reference: None,
            object_name: None,
            edit_status: None,
            editorial_update: None,
            urgency: None,
            subject_reference: None,
            category: None,
            supplemental_categories: None,
            fixture_id: None,
            keywords: None,
            content_location_code: None,
            content_location_name: None,
            release_date: None,
            release_time: None,
            expiration_date: None,
            expiration_time: None,
            special_instructions: None,
            action_advised: None,
            reference_service: None,
            reference_date: None,
            reference_number: None,
            date_created: None,
            time_created: None,
            digital_date_created: None,
            digital_time_created: None,
            originating_program: None,
            program_version: None,
            object_cycle: None,
            by_line: None,
            by_line_title: None,
            city: None,
            sub_location: None,
            province_or_state: None,
            country_or_primary_location_code: None,
            country_or_primary_location_name: None,
            original_transmission_reference: None,
            headline: None,
            credit: None,
            source: None,
            copyright_notice: None,
            contact: None,
            caption: None,
            local_caption: None,
            caption_writer: None,
            rasterized_caption: None,
            image_type: None,
            image_orientation: None,
            language_identifier: None,
            audio_type: None,
            audio_sampling_rate: None,
            audio_sampling_resolution: None,
            audio_duration: None,
            audio_outcue: None,
            job_id: None,
            master_document_id: None,
            short_document_id: None,
            unique_document_id: None,
            owner_id: None,
        }
    }
    
    pub fn set_field_value(&mut self, key: String, value: String) -> std::io::Result<()> {
        match key.as_str() {
            "image_path" => self.image_path = value,
            "model_version" => self.model_version = Some(value),
            "date_sent" => self.date_sent = Some(value),
            "time_sent" => self.time_sent = Some(value),
            "coded_character_set" => self.coded_character_set = Some(value),
            "record_version" => self.record_version = Some(value),
            "object_type_reference" => self.object_type_reference = Some(value),
            "object_attribute_reference" => self.object_attribute_reference = Some(value),
            "object_name" => self.object_name = Some(value),
            "edit_status" => self.edit_status = Some(value),
            "editorial_update" => self.editorial_update = Some(value),
            "urgency" => self.urgency = Some(value),
            "subject_reference" => self.subject_reference = Some(value),
            "category" => self.category = Some(value),
            "supplemental_categories" => self.supplemental_categories = Some(value),
            "fixture_id" => self.fixture_id = Some(value),
            "keywords" => self.keywords = Some(value),
            "content_location_code" => self.content_location_code = Some(value),
            "content_location_name" => self.content_location_name = Some(value),
            "release_date" => self.release_date = Some(value),
            "release_time" => self.release_time = Some(value),
            "expiration_date" => self.expiration_date = Some(value),
            "expiration_time" => self.expiration_time = Some(value),
            "special_instructions" => self.special_instructions = Some(value),
            "action_advised" => self.action_advised = Some(value),
            "reference_service" => self.reference_service = Some(value),
            "reference_date" => self.reference_date = Some(value),
            "reference_number" => self.reference_number = Some(value),
            "date_created" => self.date_created = Some(value),
            "time_created" => self.time_created = Some(value),
            "digital_date_created" => self.digital_date_created = Some(value),
            "digital_time_created" => self.digital_time_created = Some(value),
            "originating_program" => self.originating_program = Some(value),
            "program_version" => self.program_version = Some(value),
            "object_cycle" => self.object_cycle = Some(value),
            "by_line" => self.by_line = Some(value),
            "by_line_title" => self.by_line_title = Some(value),
            "city" => self.city = Some(value),
            "sub_location" => self.sub_location = Some(value),
            "province_or_state" => self.province_or_state = Some(value),
            "country_or_primary_location_code" => self.country_or_primary_location_code = Some(value),
            "country_or_primary_location_name" => self.country_or_primary_location_name = Some(value),
            "original_transmission_reference" => self.original_transmission_reference = Some(value),
            "headline" => self.headline = Some(value),
            "credit" => self.credit = Some(value),
            "source" => self.source = Some(value),
            "copyright_notice" => self.copyright_notice = Some(value),
            "contact" => self.contact = Some(value),
            "caption" => self.caption = Some(value),
            "local_caption" => self.local_caption = Some(value),
            "caption_writer" => self.caption_writer = Some(value),
            "rasterized_caption" => self.rasterized_caption = Some(value),
            "image_type" => self.image_type = Some(value),
            "image_orientation" => self.image_orientation = Some(value),
            "language_identifier" => self.language_identifier = Some(value),
            "audio_type" => self.audio_type = Some(value),
            "audio_sampling_rate" => self.audio_sampling_rate = Some(value),
            "audio_sampling_resolution" => self.audio_sampling_resolution = Some(value),
            "audio_duration" => self.audio_duration = Some(value),
            "audio_outcue" => self.audio_outcue = Some(value),
            "job_id" => self.job_id = Some(value),
            "master_document_id" => self.master_document_id = Some(value),
            "short_document_id" => self.short_document_id = Some(value),
            "unique_document_id" => self.unique_document_id = Some(value),
            "owner_id" => self.owner_id = Some(value),
            _ => {
                // try setting with a different case if it is in PascalCase
                if key.is_case(convert_case::Case::Pascal) {
                    return self.set_field_value(key.to_case(convert_case::Case::Snake), value)
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::NotFound, key));                    
                }
            }
        }
        Ok(())
    }
    
    pub fn get_field(&self, c: &str) -> Option<String> {
        match c {
            "image_path" => Some(self.image_path.clone()),
            "model_version" => self.model_version.clone(),
            "date_sent" => self.date_sent.clone(),
            "time_sent" => self.time_sent.clone(),
            "coded_character_set" => self.coded_character_set.clone(),
            "record_version" => self.record_version.clone(),
            "object_type_reference" => self.object_type_reference.clone(),
            "object_attribute_reference" => self.object_attribute_reference.clone(),
            "object_name" => self.object_name.clone(),
            "edit_status" => self.edit_status.clone(),
            "editorial_update" => self.editorial_update.clone(),
            "urgency" => self.urgency.clone(),
            "subject_reference" => self.subject_reference.clone(),
            "category" => self.category.clone(),
            "supplemental_categories" => self.supplemental_categories.clone(),
            "fixture_id" => self.fixture_id.clone(),
            "keywords" => self.keywords.clone(),
            "content_location_code" => self.content_location_code.clone(),
            "content_location_name" => self.content_location_name.clone(),
            "release_date" => self.release_date.clone(),
            "release_time" => self.release_time.clone(),
            "expiration_date" => self.expiration_date.clone(),
            "expiration_time" => self.expiration_time.clone(),
            "special_instructions" => self.special_instructions.clone(),
            "action_advised" => self.action_advised.clone(),
            "reference_service" => self.reference_service.clone(),
            "reference_date" => self.reference_date.clone(),
            "reference_number" => self.reference_number.clone(),
            "date_created" => self.date_created.clone(),
            "time_created" => self.time_created.clone(),
            "digital_date_created" => self.digital_date_created.clone(),
            "digital_time_created" => self.digital_time_created.clone(),
            "originating_program" => self.originating_program.clone(),
            "program_version" => self.program_version.clone(),
            "object_cycle" => self.object_cycle.clone(),
            "by_line" => self.by_line.clone(),
            "by_line_title" => self.by_line_title.clone(),
            "city" => self.city.clone(),
            "sub_location" => self.sub_location.clone(),
            "province_or_state" => self.province_or_state.clone(),
            "country_or_primary_location_code" => self.country_or_primary_location_code.clone(),
            "country_or_primary_location_name" => self.country_or_primary_location_name.clone(),
            "original_transmission_reference" => self.original_transmission_reference.clone(),
            "headline" => self.headline.clone(),
            "credit" => self.credit.clone(),
            "source" => self.source.clone(),
            "copyright_notice" => self.copyright_notice.clone(),
            "contact" => self.contact.clone(),
            "caption" => self.caption.clone(),
            "local_caption" => self.local_caption.clone(),
            "caption_writer" => self.caption_writer.clone(),
            "rasterized_caption" => self.rasterized_caption.clone(),
            "image_type" => self.image_type.clone(),
            "image_orientation" => self.image_orientation.clone(),
            "language_identifier" => self.language_identifier.clone(),
            "audio_type" => self.audio_type.clone(),
            "audio_sampling_rate" => self.audio_sampling_rate.clone(),
            "audio_sampling_resolution" => self.audio_sampling_resolution.clone(),
            "audio_duration" => self.audio_duration.clone(),
            "audio_outcue" => self.audio_outcue.clone(),
            "job_id" => self.job_id.clone(),
            "master_document_id" => self.master_document_id.clone(),
            "short_document_id" => self.short_document_id.clone(),
            "unique_document_id" => self.unique_document_id.clone(),
            "owner_id" => self.owner_id.clone(),
            _ => {
                // try getting with a different case if it is in PascalCase
                if c.is_case(convert_case::Case::Pascal) {
                    self.get_field(&c.to_case(convert_case::Case::Snake))
                } else {
                    None
                }
            },
        }
    }

    pub fn is_none(&self) -> bool {
        self.model_version.is_none() &&
        self.date_sent.is_none() &&
        self.time_sent.is_none() &&
        self.coded_character_set.is_none() &&
        self.record_version.is_none() &&
        self.object_type_reference.is_none() &&
        self.object_attribute_reference.is_none() &&
        self.object_name.is_none() &&
        self.edit_status.is_none() &&
        self.editorial_update.is_none() &&
        self.urgency.is_none() &&
        self.subject_reference.is_none() &&
        self.category.is_none() &&
        self.supplemental_categories.is_none() &&
        self.fixture_id.is_none() &&
        self.keywords.is_none() &&
        self.content_location_code.is_none() &&
        self.content_location_name.is_none() &&
        self.release_date.is_none() &&
        self.release_time.is_none() &&
        self.expiration_date.is_none() &&
        self.expiration_time.is_none() &&
        self.special_instructions.is_none() &&
        self.action_advised.is_none() &&
        self.reference_service.is_none() &&
        self.reference_date.is_none() &&
        self.reference_number.is_none() &&
        self.date_created.is_none() &&
        self.time_created.is_none() &&
        self.digital_date_created.is_none() &&
        self.digital_time_created.is_none() &&
        self.originating_program.is_none() &&
        self.program_version.is_none() &&
        self.object_cycle.is_none() &&
        self.by_line.is_none() &&
        self.by_line_title.is_none() &&
        self.city.is_none() &&
        self.sub_location.is_none() &&
        self.province_or_state.is_none() &&
        self.country_or_primary_location_code.is_none() &&
        self.country_or_primary_location_name.is_none() &&
        self.original_transmission_reference.is_none() &&
        self.headline.is_none() &&
        self.credit.is_none() &&
        self.source.is_none() &&
        self.copyright_notice.is_none() &&
        self.contact.is_none() &&
        self.caption.is_none() &&
        self.local_caption.is_none() &&
        self.caption_writer.is_none() &&
        self.rasterized_caption.is_none() &&
        self.image_type.is_none() &&
        self.image_orientation.is_none() &&
        self.language_identifier.is_none() &&
        self.audio_type.is_none() &&
        self.audio_sampling_rate.is_none() &&
        self.audio_sampling_resolution.is_none() &&
        self.audio_duration.is_none() &&
        self.audio_outcue.is_none() &&
        self.job_id.is_none() &&
        self.master_document_id.is_none() &&
        self.short_document_id.is_none() &&
        self.unique_document_id.is_none() &&
        self.owner_id.is_none()
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_IPTC_COLUMNS_JSON).unwrap()
    }
}

impl std::fmt::Display for ImageIptc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "iptc: {:?}", self)
    }
}