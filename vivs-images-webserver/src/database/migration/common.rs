use sqlx::Row;



pub struct Migration {
    pub version: i32,
    pub description: String,
    pub applied_at: String,
}

impl Migration {
    pub fn new_from_db(row: &sqlx::sqlite::SqliteRow) -> Self {
        let version: i32 = row.try_get("version").unwrap_or(0);
        let description: String = row.try_get("description").unwrap_or_default();
        let applied_at: String = row.try_get("applied_at").unwrap_or_default();

        Migration {
            version,
            description,
            applied_at,
        }
    }

    pub fn new_from_file(version: i32, description: &str) -> Self {
        Migration {
            version,
            description: description.to_string(),
            applied_at: "".to_string(),
        }
    }
}