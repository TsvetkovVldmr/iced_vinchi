use std::path::PathBuf;
use iced::{alignment, Element, Length, Settings};

use iced::Sandbox;
use iced::widget::{Button, Text, Column, Container, column, text, Row, container, row};


use iced::{Color, Command, Subscription};

use iced::theme::Theme;

use rfd::FileDialog;

// TODO: 1. Убрать лишние контролы, переименовать типы
// TODO: 2. Вынести всэ касающееся песен в отдельный файл
// TODO: 3. Приделать меню сверху
// TODO: 4. Приделать кнопки плэй и стоп снизу
// TODO: 5. Приделать кнопка next previous рядом
// TODO: 6. Добавить проигрывание
// TODO: 7. Progress bar
// TODO: 8. Customization view
// TODO:
// TODO:
// TODO:





fn main() -> Result<(), iced::Error> {
    Player::run(Settings::default())
}



struct Player {
    theme: Theme,
    songs: Vec<Song>
}

#[derive(Debug, Clone)]
enum Message {
    Open,
    SongMessage(SongMessage)
}

impl Sandbox for Player {
    type Message = Message;


    fn new() -> Self {
        Player { theme: Theme::Dark, songs: vec![] }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Open => {
                let files = FileDialog::new()
                    .set_directory("/")
                    .pick_files();

                match files {
                    Some(files) => {

                        let songs = files
                            .into_iter()
                            .map(move |p| Song { name: p.file_name().unwrap().to_str().unwrap().to_string(), path: p });

                        for song in songs {
                            self.songs.push(song);
                        }
                    }
                    None => {}
                }
            },
            Message::SongMessage(song) => {}
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        let open = Button::new("Open").on_press(Message::Open);

        let test: Vec<Element<_>> = self.songs.iter().map(|song| song.view().map(move |msg| Message::SongMessage(msg))).collect();

        let songs = column(test);


        let col = Column::new().push(open);

        let row = Row::new().push(songs).push(col);

        Container::new(row).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

struct Song {
    name: String,
    path: PathBuf
}

impl Song {
    fn view(&self) -> Element<SongMessage> {
        let label = text(&self.name);

        row![label]
            .spacing(20)
            .into()
    }
}

#[derive(Debug, Clone)]
enum SongMessage{
    Ok
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
        .width(Length::Fill)
        .height(200)
        .center_y()
        .into()
}