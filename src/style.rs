use iced::application::{self, StyleSheet};
use iced::border::Radius;
use iced::color;
use iced::widget::text_input;
use iced::{Border, Color};

#[derive(Default)]
pub struct Theme;

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
        application::Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(iced::Color::BLACK),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(0.0),
            },
            icon_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(iced::Color::BLACK),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(0.0),
            },
            icon_color: Color::TRANSPARENT,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0x292929)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        iced::Color::WHITE
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0xff0099)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        color!(0xffff00)
    }

    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(iced::Color::WHITE),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(0.0),
            },
            icon_color: Color::TRANSPARENT,
        }
    }
}
