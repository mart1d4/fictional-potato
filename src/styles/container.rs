use iced::{Background, Border, Color, Shadow, Theme, border::Radius, widget::container};

use crate::colors::AppColorBackground;

pub fn container_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(Background::Color(AppColorBackground::Secondary.into())),
        border: Border {
            radius: Radius {
                top_left: 4.0,
                top_right: 4.0,
                bottom_left: 4.0,
                bottom_right: 4.0,
            },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}
