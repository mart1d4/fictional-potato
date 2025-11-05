use crate::colors::{AppColorBackground, AppColorForeground, AppColorMain};

use iced::{
    Background, Border, Color, Element,
    Length::Fill,
    Padding, Radians, Task,
    border::Radius,
    widget::{button, column, container, horizontal_space, row, text, text_input, vertical_space},
};
use validator::Validate;

#[derive(Debug, Clone, Default, Validate)]
pub struct State {
    pub is_loading: bool,
}

impl State {
    pub fn new() -> Self {
        Self { is_loading: false }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    DoSomething,
}

pub fn view(state: &State) -> Element<'_, Message> {
    row![
        container(
            column![
                button(text("A").size(18).width(Fill).height(Fill).center())
                    .width(40)
                    .height(40)
                    .style(|_t, _s| {
                        button::Style {
                            border: Border {
                                radius: 10.0.into(),
                                ..Default::default()
                            },
                            background: Some(AppColorMain::Secondary.to_bg()),
                            ..Default::default()
                        }
                    }),
                vertical_space(),
            ]
            .spacing(12)
        )
        .style(|_t| {
            container::Style {
                background: Some(AppColorBackground::Primary.to_bg()),
                ..Default::default()
            }
        })
        .center_x(72)
        .padding(Padding {
            top: 12.0,
            bottom: 12.0,
            left: 16.0,
            right: 16.0,
        })
        .height(Fill),
        container(
            column![
                button(
                    text("Find or start a conversation")
                        .width(Fill)
                        .height(40)
                        .size(14)
                        .color(Color::from(AppColorForeground::Secondary))
                        .center()
                )
                .padding(0)
                .width(Fill)
                .style(|_t, _s| {
                    button::Style {
                        background: Some(AppColorBackground::SurfacePrimary.to_bg()),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }),
                vertical_space()
            ]
            .spacing(12)
        )
        .width(260)
        .height(Fill)
        .padding(12)
        .style(|_t| {
            container::Style {
                background: Some(AppColorBackground::Secondary.to_bg()),
                ..Default::default()
            }
        }),
        container(text("Messagesss"))
            .width(Fill)
            .height(Fill)
            .style(|_t| {
                container::Style {
                    background: Some(AppColorBackground::Tertiary.to_bg()),
                    ..Default::default()
                }
            })
    ]
    .width(Fill)
    .height(Fill)
    .into()
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        _ => Task::none(),
    }
}
