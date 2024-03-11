mod style;

use iced::keyboard::key::Named;
use iced::widget::text::LineHeight;
use iced::widget::text_input::{self, focus};
use iced::widget::{Row, TextInput};
use iced::window::Position;
use iced::{
    event, executor, keyboard, window, Alignment, Application, Command, Element, Event, Padding,
    Pixels, Point, Settings, Size, Subscription,
};
use once_cell::sync::Lazy;
use style::Theme;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
    let (w, _) = rdev::display_size().unwrap();
    let settings = Settings {
        window: window::Settings {
            size: Size::new(w as f32, 25.0),
            decorations: false,
            position: Position::Specific(Point::new(0.0, 0.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    App::run(settings)
}

#[derive(Default)]
struct App {
    query: String,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    Command,
    Close,
    None,
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn theme(&self) -> Self::Theme {
        Theme
    }

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), focus(INPUT_ID.clone()))
    }

    fn title(&self) -> String {
        String::from(env!("CARGO_PKG_NAME"))
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen().map(|event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                location: _,
                modifiers: _,
                text: _,
            }) => {
                if key == keyboard::Key::Named(Named::Escape) {
                    Message::Close
                } else {
                    Message::None
                }
            }
            _ => Message::None,
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextInputChanged(txt) => {
                self.query = txt;
                Command::none()
            }
            Message::Command => {
                let cmd = &self.query;
                println!("{cmd}");
                window::close(window::Id::MAIN)
            }
            Message::Close => window::close(window::Id::MAIN),
            Message::None => Command::none(),
        }
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        let input = TextInput::new("Search...", self.query.as_str())
            .line_height(LineHeight::Absolute(Pixels(25.0)))
            .padding(Padding::from([0.0, 10.0]))
            .on_input(Message::TextInputChanged)
            .on_submit(Message::Command)
            .id(INPUT_ID.clone());

        Row::new().push(input).align_items(Alignment::Center).into()
    }
}
