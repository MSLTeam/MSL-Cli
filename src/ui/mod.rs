pub mod home;
pub mod components;

use ratatui::prelude::*;
use crate::core::api::{Announcement};

pub struct AppState {
    pub should_quit: bool,
    pub selected_tab: usize,
    pub home_data: Option<Announcement>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            selected_tab: 0,
            home_data: None,
        }
    }
}

pub fn render(f: &mut Frame, state: &AppState) {
    home::render(f, state);
}
