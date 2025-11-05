use crate::{
    RUBIK,
    colors::{AppColorForeground, AppColorMain},
    components::styled_input,
    styles::{button_style, container_style},
    utils::set_token_from_secure_storage,
};
use std::collections::HashMap;
use turbo::{auth::AuthResponse, errors::ResponseError, types::PublicUser};

use iced::{
    Color, Element,
    Length::{self, Fill},
    Task,
    widget::{button, column, container, row, text},
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
    LoginSuccess(PublicUser),
    LoginFailed(String),
    RegisterInstead,
    RequestScreenChange(super::CurrentScreen),
}

pub fn view(state: &State) -> Element<'_, Message> {
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
    .style(button_style)
    .padding(10);

    let register_link = button(
        text("Register")
            .color(Color::from(AppColorMain::Secondary))
            .size(12),
    )
    .on_press(Message::RegisterInstead)
    .style(button::text)
    .height(14)
    .padding(0);

    container(
        column![
            text("Log in").font(RUBIK).size(24).width(Fill).center(),
            styled_input(
                "Username or Email",
                &state.identifier,
                state.identifier_error.as_deref(),
                None,
                Message::UsernameInputChanged,
                //,
                None,
                Some(true),
            ),
            styled_input(
                "Password",
                &state.password,
                state.password_error.as_deref(),
                None,
                Message::PasswordInputChanged,
                //,
                Some(true),
                Some(true),
            ),
            column![
                login_button,
                row![
                    text("Don't have an account? ")
                        .size(12)
                        .color(Color::from(AppColorForeground::SubtextPrimary)),
                    register_link
                ],
            ]
            .spacing(8)
        ]
        .width(Length::Fixed(550.0))
        .padding(24)
        .spacing(24),
    )
    .style(container_style)
    .into()
}

pub async fn perform_login(identifier: String, password: String) -> Result<PublicUser, String> {
    println!("Attempting login for user with identifier: {}.", identifier);

    let mut map = HashMap::new();
    map.insert("identifier", identifier);
    map.insert("password", password);

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8585/auth/login")
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
        set_token_from_secure_storage(Some(success_body.refresh_token)).map_err(|e| "no")?;
        Ok(success_body.user)
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
            Task::perform(
                perform_login(state.identifier.clone(), state.password.clone()),
                |result| match result {
                    Ok(user) => Message::LoginSuccess(user),
                    Err(e) => Message::LoginFailed(e),
                },
            )
        }
        Message::LoginSuccess(_) => {
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
