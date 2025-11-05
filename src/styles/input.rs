use iced::{
    Background, Border, Color, Theme,
    widget::text_input::{self, Status},
};

use crate::{
    colors::{AppColorBackground, AppColorForeground, AppColorMain},
    constants::BORDER_RADIUS,
};

pub fn input_style(_theme: &Theme, _status: Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::from(AppColorBackground::Tertiary)),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        placeholder: Color::from(AppColorForeground::SubtextTertiary).scale_alpha(0.8),
        value: AppColorForeground::Secondary.into(),
        selection: Color::from(AppColorMain::Primary).scale_alpha(0.2),
    }
}
