
pub const SQL_CREATE_IMAGE_TAGS: &str = r#"

CREATE TABLE image_tags (
    image_tag_id INTEGER NOT NULL PRIMARY KEY,
    image_path TEXT NOT NULL,
    tag_name TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_image_tags_unique ON image_tags(image_path, tag_name);
CREATE INDEX idx_image_tags_image_path ON image_tags(image_path);
CREATE INDEX idx_image_tags_tag_name ON image_tags(tag_name);



CREATE TABLE tags (
    tag_name TEXT NOT NULL PRIMARY KEY,
    tag_label TEXT NOT NULL,
    tag_description TEXT NOT NULL
);

CREATE INDEX idx_tags_tag_name ON tags(tag_name);

"#;