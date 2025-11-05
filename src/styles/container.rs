use iced::{Background, Border, Color, Shadow, Theme, widget::container};

use crate::{colors::AppColorBackground, constants::BORDER_RADIUS};

pub fn container_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(Background::Color(Color::from(
            AppColorBackground::SurfacePrimary,
        ))),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow::default(),
    }
}
