use serde::Deserialize;
use url_escape::decode_to_string;
use std::collections::HashMap;

use crate::{get_file_from_exe_dir, models::image::Image};

#[derive(Clone, Debug, Deserialize)]
pub struct SearchParamFieldInput {
    pub name: String,
    pub value: Option<String>,
}

impl SearchParamFieldInput {
    pub fn to_output(&self, field_meta: SearchParamFieldMeta) -> SearchParamFieldOutput {
        SearchParamFieldOutput {
            field_meta,
            field_input: self.clone(),
        }
    }

    // Helper function to add HTML parameter
    pub fn add_html_param<T: ToString>(
        &self,
        params: &mut HashMap<String, String>,
        field: &Option<T>
    ) {
        if let Some(value) = field {
            params.insert(self.name.clone(), value.to_string());
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchParamFieldList {
    pub fields: Vec<SearchParamFieldOutput>,
}

impl SearchParamFieldList {
    pub fn new(fields: Vec<SearchParamFieldOutput>) -> Self {
        Self { fields }
    }

    pub fn to_html(&self) -> String {
        self.fields.iter().map(|field| field.to_html()).collect::<Vec<_>>().join("\n")
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchParamFieldMeta {
    pub name: String,
    pub label: String,
    pub input_type: String,
    pub placeholder: String,
    pub sql_field: Option<String>,
    pub default: Option<String>,
    pub is_regular: bool,
    pub is_advanced: bool,
    pub is_for_display: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchParamFieldOutput {
    pub field_meta: SearchParamFieldMeta,
    pub field_input: SearchParamFieldInput,
}

impl SearchParamFieldOutput {
    pub fn to_html(&self) -> String {
        let value = self.field_input.value.as_deref().unwrap_or("");
        let other_attributes   = match self.field_meta.input_type.as_str() {
            "number" => "step=\"any\"",
            _ => "",
        };
        format!(
            r#"<div class="form-group">
                <label for="{}">{}</label>
                <input type="{}" id="{}" name="{}" placeholder="{}" value="{}" {}/>
            </div>"#,
            self.field_meta.name, self.field_meta.label, self.field_meta.input_type, self.field_meta.name, self.field_meta.name, self.field_meta.placeholder, value, other_attributes
        )
    }

    pub fn get_sql_comparison_operator(&self) -> Option<&'static str> {
        self.field_meta.sql_field.as_ref().and_then(|_| {
            let comparison_operator = match self.field_meta.input_type.as_str() {
                "number" | "date" => {
                    if self.field_meta.name.ends_with("_min") {
                        ">="
                    } else if self.field_meta.name.ends_with("_max") {
                        "<="
                    } else {
                        "="
                    }
                },
                "text" => "LIKE",
                _ => "=",
            };
            Some(comparison_operator)
        })
    }

    pub fn add_sql_condition(&self, params: &mut HashMap<String, String>) {
        if let Some(sql_field) = &self.field_meta.sql_field {
            if let Some(comparison_operator) = self.get_sql_comparison_operator() {
                let v = self.field_input.value.as_ref().or(self.field_meta.default.as_ref());

                if let Some(value) = v {
                    if !value.is_empty() {
                        let formatted_value = if comparison_operator == "LIKE" {
                            format!("%{}%", value)
                        } else {
                            value.clone()
                        };
                        params.insert(format!("{} {} ?", sql_field, comparison_operator), formatted_value);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub fields: Vec<SearchParamFieldOutput>,
}

// json list of SearchParamFieldMeta
pub const SEARCH_PARAMS_JSON_PATH: &str = "search_params.json";

impl SearchParams {
    pub fn get_meta() -> Vec<SearchParamFieldMeta> {
        let path = get_file_from_exe_dir(SEARCH_PARAMS_JSON_PATH);
        let json = std::fs::read_to_string(path).unwrap();
        serde_json::from_str::<Vec<SearchParamFieldMeta>>(&json).unwrap()
    }

    pub fn default() -> Self {
        Self {
            fields: vec![],
        }
    }

    pub fn get_query(&self) -> Option<String> {
        self.get_field_value("query")
    }

    pub fn get_use_simple_view(&self) -> Option<bool> {
        self.get_field_value("use_simple_view").map(|value| value == "true")
    }

    pub fn get_category(&self) -> Option<String> {
        self.get_field_value("category")
    }

    pub fn get_category_value(&self) -> Option<String> {
        self.get_field_value("category_value")
    }

    pub fn get_limit(&self) -> Option<i32> {
        self.get_field_value_or_default("limit").and_then(|v| v.parse::<i32>().ok())
    }

    pub fn get_offset(&self) -> Option<i32> {
        self.get_field_value("offset").and_then(|v| v.parse::<i32>().ok())
    }

    pub fn into_sql_query_params(&self) -> Vec<HashMap<String, String>> {
        let mut param_groups = vec![];
        
        // Handle query search across multiple fields
        if let Some(query) = &self.get_field_value("query") {
            let search_pattern = format!("%{}%", query);
            let mut param_group = HashMap::new();
            for text_field in &self.fields {
                if text_field.field_meta.input_type == "text" {
                    if let Some(sql_field) = &text_field.field_meta.sql_field {
                        param_group.insert(format!("{} LIKE ?", sql_field), search_pattern.clone());
                    }
                }
            }
            param_groups.push(param_group);
        }

        let mut params = HashMap::new();
        for field in &self.fields {
            field.add_sql_condition(&mut params);
        }
        param_groups.push(params);
        
        param_groups
    }

    pub fn into_html_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for field in &self.fields {
            if let Some(v_in) = field.field_input.value.as_ref() {
                if let Some(v_default) = field.field_meta.default.as_ref() {
                    if v_in == v_default {
                        continue;
                    }
                } else if v_in.is_empty() {
                    continue;
                }
            } else if let Some(v_default) = field.field_meta.default.as_ref() {
                if v_default.is_empty() {
                    continue;
                }
            }
            field.field_input.add_html_param(&mut params, &field.field_input.value);
        }
        params
    }

    // Helper function to format field for string representation
    fn format_field<T: ToString>(parts: &mut Vec<String>, field: &Option<T>, label: &str) {
        if let Some(value) = field {
            parts.push(format!("{}: {}", label, value.to_string()));
        }
    }

    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        for field in &self.fields {
            if !field.field_meta.is_for_display {
                Self::format_field(&mut parts, &field.field_input.value, &field.field_meta.label);
            }
        }

        parts.join(", ")
    }

    pub fn set_field_value(&mut self, name: &str, value: Option<String>) {
        let field_input = SearchParamFieldInput { name: name.to_string(), value };
        let field_index = self.fields.iter().position(|x| x.field_meta.name == name);
        if let Some(field_index) = field_index {
            let x = self.fields.remove(field_index);
            self.fields.push(SearchParamFieldOutput { field_meta: x.field_meta, field_input });
        }
    }

    fn get_field_value(&self, name: &str) -> Option<String> {
        for field in &self.fields {
            if let Some(v_in) = field.field_input.value.as_ref() {
                if let Some(v_default) = field.field_meta.default.as_ref() {
                    if v_in == v_default {
                        continue;
                    }
                } else if v_in.is_empty() {
                    continue;
                }
            } else if let Some(v_default) = field.field_meta.default.as_ref() {
                if v_default.is_empty() {
                    continue;
                }
            }

            if field.field_meta.name == name {
                return field.field_input.value.clone();
            }
        }
        None
    }

    fn get_field_value_or_default(&self, name: &str) -> Option<String> {
        for field in &self.fields {
            if field.field_meta.name == name {
                let mut v_in = field.field_input.value.as_ref();
                if let Some(v) = v_in {
                    if v.is_empty() {
                        v_in = field.field_meta.default.as_ref();
                    }
                } else {
                    v_in = field.field_meta.default.as_ref();
                }
                return v_in.cloned();
            }
        }
        None
    }

    pub fn new_from_hashmap(params: &HashMap<String, String>) -> SearchParams {
        let mut search_params = SearchParams::default();
        for field in Self::get_meta() {
            let mut value = params.get(&field.name).cloned();
            if let Some(v_in) = value.as_ref() {
                if v_in.is_empty() && field.default.is_none() {
                    value = None;
                }
            }

            let field_input = SearchParamFieldInput {
                name: field.name.clone(),
                value,
            };
            let field_output = field_input.to_output(field.clone());
            search_params.fields.push(field_output);
        }
        search_params
    }

    pub fn new_from_querystring(query_string: &str) -> SearchParams {
        // Parse query string into HashMap
        let mut params: HashMap<String, String> = query_string
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?.to_string();
                let value = parts.next().unwrap_or("").to_string();
                let mut decoded_value = String::new();
                decode_to_string(value, &mut decoded_value);
                Some((key, decoded_value))
            })
            .collect();
        
        // Handle checkbox fields and default value comparison
        for field in Self::get_meta() {
            if field.input_type == "checkbox" {
                // For checkboxes, presence means "true"
                if params.contains_key(&field.name) {
                    params.insert(field.name.clone(), "true".to_string());
                }
            } else if let Some(value) = params.get(&field.name) {
                // For other fields, remove if value equals default
                if let Some(default_val) = field.default {
                    if value == &default_val {
                        params.remove(&field.name);
                    }
                }
            }
        }
        
        // Reuse the hashmap function
        Self::new_from_hashmap(&params)
    }
    
    pub fn get_columns_to_display(&self) -> Option<Vec<String>> {
        let columns: Vec<String> = self.get_field_value("columns_to_display")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        
        if columns.is_empty() {
            None
        } else {
            Some(columns)
        }
    }
    
    pub fn get_column_titles(columns: &[String]) -> Vec<String> {
        let all_meta = Image::get_meta();
        columns.iter().filter_map(|c| {
            match c.as_str() {
                "thumbnail" => Some("Thumbnail".to_string()),
                "path" => Some("Path".to_string()),
                _ => all_meta.iter()
                        .find(|m| m.name == *c)
                        .map(|m| m.label.clone())
                        
            }
        }).collect()
    }
}