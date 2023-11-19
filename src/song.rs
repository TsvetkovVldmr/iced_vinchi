use std::path::PathBuf;
use iced::Element;
use iced::widget::{row, text};

pub struct Song {
    pub name: String,
    pub path: PathBuf
}

impl Song {
    pub fn view(&self) -> Element<SongMessage> {
        let label = text(&self.name);

        row![label]
            .spacing(20)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum SongMessage{
    Ok
}