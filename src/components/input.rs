use iced::{
    Color, Element,
    Length::Fill,
    widget::{column, row, text, text_input},
};

use crate::{RUBIK, colors::AppColorStatus, styles::input_style};

pub fn styled_input(
    label: &str,
    value: &str,
    error: Option<&str>,
    description: Option<&str>,
    //on_input: T,
    //on_submit: T,
    is_secure: Option<bool>,
    is_required: Option<bool>,
) -> Element<'_, T> {
    let password_confirm_input = text_input(label, value)
        //.on_input(on_input)
        //.on_submit(on_submit)
        .line_height(1.5)
        .width(Fill)
        .secure(is_secure.unwrap_or(false))
        .style(|theme, _status| input_style(theme))
        .padding(10);

    column![
        row![
            text!("{}", label.to_string().to_uppercase().as_str())
                .size(12)
                .font(RUBIK),
            text!(
                "{} {}",
                if is_secure.unwrap_or(false) || error.is_some() {
                    "* "
                } else {
                    ""
                },
                if let Some(err) = error { err } else { "" }
            )
            .color(Color::from(AppColorStatus::Failure))
            .size(14)
        ]
        .spacing(4),
        password_confirm_input
    ]
    .spacing(4)
    .into()
}
