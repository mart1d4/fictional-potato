use iced::{Background, Border, Color, Theme, border::Radius, widget::text_input};

use crate::colors::{AppColorBackground, AppColorForeground, AppColorMain};

pub fn input_style(_theme: &Theme) -> text_input::Style {
    text_input::Style {
        background: Background::Color(AppColorBackground::Tertiary.into()),
        border: Border {
            radius: Radius {
                top_left: 3.0,
                top_right: 3.0,
                bottom_left: 3.0,
                bottom_right: 3.0,
            },
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        placeholder: AppColorForeground::SubtextSecondary.into(),
        value: AppColorForeground::Secondary.into(),
        selection: Color::from(AppColorMain::Primary).scale_alpha(0.5),
    }
}
