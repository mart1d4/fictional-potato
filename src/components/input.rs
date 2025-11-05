use iced::{
    Color, Element,
    Length::Fill,
    widget::{column, row, text, text_input},
};

use crate::{RUBIK, colors::AppColorStatus, styles::input_style};

pub fn styled_input<'a, M>(
    label: &str,
    value: &str,
    error: Option<&str>,
    description: Option<&str>,
    on_input: impl Fn(String) -> M + 'a + Clone,
    //on_submit: impl Fn() -> M + 'a + Clone,
    is_secure: Option<bool>,
    is_required: Option<bool>,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let password_confirm_input = text_input(label, value)
        .on_input(on_input)
        //.on_submit(on_submit)
        .line_height(1.2)
        .width(Fill)
        .secure(is_secure.unwrap_or(false))
        .style(input_style)
        .padding(10);

    let show_star = is_required.unwrap_or(false) && !error.is_some();
    let show_dash = error.is_some();

    column![
        row![
            text!("{}", label).size(13).font(RUBIK),
            text!(
                "{} {}",
                if show_star {
                    "*"
                } else if show_dash {
                    "-"
                } else {
                    ""
                },
                if let Some(err) = error { err } else { "" }
            )
            .color(Color::from(AppColorStatus::Failure))
            .size(13)
        ]
        .spacing(4),
        password_confirm_input
    ]
    .spacing(4)
    .into()
}
