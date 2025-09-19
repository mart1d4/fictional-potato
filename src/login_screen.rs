use crate::{
    RUBIK,
    colors::{AppColorForeground, AppColorMain},
    components::{container_style, input_style},
};
use std::collections::HashMap;
use turbo::{auth::AuthResponse, errors::ResponseError};

use iced::{
    Color, Element,
    Length::{self, Fill},
    Task,
    widget::{button, column, container, row, text, text_input},
};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub identifier: String,
    pub password: String,
    pub is_loading: bool,
    pub identifier_error: Option<String>,
    pub password_error: Option<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            identifier: String::new(),
            password: String::new(),
            is_loading: false,
            identifier_error: None,
            password_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UsernameInputChanged(String),
    PasswordInputChanged(String),
    LoginButtonPressed,
    LoginSuccess,
    LoginFailed(String),
    RegisterInstead,
    RequestScreenChange(super::CurrentScreen),
}

pub fn view(state: &State) -> Element<'_, Message> {
    let identifier_input = text_input("Username or Email", &state.identifier)
        .on_input(Message::UsernameInputChanged)
        .on_submit(Message::LoginButtonPressed)
        .line_height(1.5)
        .width(Fill)
        .style(|theme, _status| input_style(theme))
        .padding(10);

    let password_input = text_input("Password", &state.password)
        .on_input(Message::PasswordInputChanged)
        .on_submit(Message::LoginButtonPressed)
        .line_height(1.5)
        .width(Fill)
        .secure(true)
        .style(|theme, _status| input_style(theme))
        .padding(10);

    let login_button = button(
        text(if state.is_loading {
            "Loging in…"
        } else {
            "Login"
        })
        .width(Fill)
        .center(),
    )
    .on_press(Message::LoginButtonPressed)
    .height(44)
    .width(Fill)
    .style(button::primary)
    .padding(10);

    let register_link = button(
        text("Register")
            .color(Color::from(AppColorMain::Primary))
            .size(14),
    )
    .on_press(Message::RegisterInstead)
    .style(button::text)
    .height(14)
    .padding(0);

    container(
        column![
            text("Log in").font(RUBIK).size(30).center(),
            column![
                row![
                    text("USERNAME OR EMAIL").size(12).font(RUBIK),
                    if let Some(err) = &state.identifier_error {
                        text("* ".to_string() + err.as_str())
                            .color(Color::parse("#f38ba8").unwrap_or(Color::BLACK))
                            .size(14)
                    } else {
                        text("*")
                            .color(Color::parse("#f38ba8").unwrap_or(Color::BLACK))
                            .size(14)
                    }
                ]
                .spacing(4),
                identifier_input
            ]
            .spacing(4),
            column![
                row![
                    text("PASSWORD").size(12),
                    if let Some(err) = &state.password_error {
                        text("* ".to_string() + err.as_str())
                            .color(Color::parse("#f38ba8").unwrap_or(Color::BLACK))
                            .size(14)
                    } else {
                        text("*")
                            .color(Color::parse("#f38ba8").unwrap_or(Color::BLACK))
                            .size(14)
                    }
                ]
                .spacing(4),
                password_input
            ]
            .spacing(4),
            column![
                login_button,
                row![
                    text("Don't have an account? ")
                        .size(14)
                        .color(Color::from(AppColorForeground::Tertiary)),
                    register_link
                ],
            ]
            .spacing(12)
        ]
        .width(Length::Fixed(550.0))
        .padding(24)
        .spacing(24),
    )
    .style(|theme| container_style(theme))
    .into()
}

pub async fn perform_login(identifier: String, password: String) -> Result<(), String> {
    println!("Attempting login for user with identifier: {}.", identifier);

    let mut map = HashMap::new();
    map.insert("identifier", identifier.clone());
    map.insert("password", password.clone());

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/login")
        .json(&map)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if res.status().is_success() {
        let success_body = res
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse successful login: {}", e))?;

        println!("res: {success_body:?}");
        Ok(())
    } else {
        let error_body = res
            .json::<ResponseError>()
            .await
            .map_err(|e| format!("Failed to parse error response: {}", e))?;

        Err(error_body.error.message)
    }
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::UsernameInputChanged(value) => {
            state.identifier = value;
            state.identifier_error = None;
            Task::none()
        }
        Message::PasswordInputChanged(value) => {
            state.password = value;
            state.password_error = None;
            Task::none()
        }
        Message::LoginButtonPressed => {
            state.is_loading = true;
            let identifier = state.identifier.clone();
            let password = state.password.clone();
            Task::perform(perform_login(identifier, password), |result| match result {
                Ok(_) => Message::LoginSuccess,
                Err(e) => Message::LoginFailed(e),
            })
        }
        Message::LoginSuccess => {
            state.is_loading = false;
            Task::none()
        }
        Message::LoginFailed(error) => {
            state.is_loading = false;
            state.identifier_error = Some(error.clone());
            state.password_error = Some(error);
            Task::none()
        }
        Message::RegisterInstead => {
            state.is_loading = false;
            Task::done(Message::RequestScreenChange(
                super::CurrentScreen::Register(super::register_screen::State::new()),
            ))
        }
        _ => Task::none(),
    }
}
