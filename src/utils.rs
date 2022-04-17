use std::path::PathBuf;
use chrono::offset::Utc;

/// creates an unique file_path with a timestamp and with the title
pub fn title_to_path(title: &str) -> PathBuf {
    let timestamp = Utc::now().format("%Y%m%d%%H%M%S");
    let formated_title: String = title.to_lowercase().chars().map(|chr| {
        match chr {
            c if c.is_whitespace() => '_',
            '-' => '_',
            _ => chr,
        }
    }).collect();
    format!("{}-{}.md", timestamp, formated_title).into()
}

