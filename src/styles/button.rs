use iced::{
    Background, Border, Color, Shadow, Theme,
    widget::button::{self, Status},
};

use crate::{
    colors::{AppColorBackground, AppColorMain},
    constants::BORDER_RADIUS_SMALL,
};

pub fn button_style(_theme: &Theme, status: Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(match status {
            Status::Active => Color::from(AppColorMain::Secondary),
            Status::Hovered => Color::from(AppColorMain::Secondary).scale_alpha(0.85),
            Status::Pressed => Color::from(AppColorMain::Secondary).scale_alpha(0.7),
            Status::Disabled => Color::from(AppColorMain::Secondary).scale_alpha(0.35),
        })),
        border: Border {
            radius: BORDER_RADIUS_SMALL.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        text_color: Color::from(AppColorBackground::Primary),
        shadow: Shadow::default(),
    }
}
