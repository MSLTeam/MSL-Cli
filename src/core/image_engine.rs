use crate::config::ImageMode;
use ratatui_image::picker::Picker;

pub struct ImageEngine;

impl ImageEngine {
    pub fn probe_supported_mode() -> ImageMode {
        match Picker::from_query_stdio() {
            Ok(_) => ImageMode::Protocol,
            Err(_) => ImageMode::Ascii,
        }
    }
}