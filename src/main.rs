pub mod song;

use iced::{alignment, Element, Length, Settings};
use iced::widget::{Button,  Column, Container, column, text, Row, container};


use iced::{Sandbox, Color};

use iced::theme::Theme;

use rfd::FileDialog;
use crate::song::{Song, SongMessage};

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

                if let Some(files) = files {

                    let songs = files
                        .into_iter()
                        .map(move |p| Song { name: p.file_name().unwrap().to_str().unwrap().to_string(), path: p });

                    for song in songs {
                        self.songs.push(song);
                    }
                }
            },
            Message::SongMessage(_) => {}
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {

        let open = Button::new("Open").on_press(Message::Open);

        let test: Vec<Element<_>> = self.songs.iter().map(|song| song.view().map(Message::SongMessage)).collect();

        let songs = column(test);


        let col = Column::new().push(open);

        let row = Row::new().push(songs).push(col);

        Container::new(row).center_x().center_y().width(Length::Fill).height(Length::Fill).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
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