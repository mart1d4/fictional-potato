use crate::{
    RUBIK,
    colors::{AppColorForeground, AppColorMain, AppColorStatus},
    styles::{container_style, input_style},
};
use std::collections::HashMap;
use turbo::{auth::AuthResponse, errors::ResponseError, types::PublicUser};

use iced::{
    Color, Element,
    Length::{self, Fill},
    Task,
    widget::{button, column, container, row, text, text_input},
};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub is_loading: bool,
    pub username_error: Option<String>,
    pub password_error: Option<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            password_confirm: String::new(),
            is_loading: false,
            username_error: None,
            password_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    UsernameInputChanged(String),
    PasswordInputChanged(String),
    PasswordConfirmInputChanged(String),
    RegisterButtonPressed,
    RegisterSuccess(PublicUser),
    RegisterFailed(String),
    LoginInstead,
    RequestScreenChange(super::CurrentScreen),
}

pub fn view(state: &State) -> Element<'_, Message> {
    let username_input = text_input("Username", &state.username)
        .on_input(Message::UsernameInputChanged)
        .on_submit(Message::RegisterButtonPressed)
        .line_height(1.5)
        .width(Fill)
        .style(|theme, _status| input_style(theme))
        .padding(10);

    let password_input = text_input("Password", &state.password)
        .on_input(Message::PasswordInputChanged)
        .on_submit(Message::RegisterButtonPressed)
        .line_height(1.5)
        .width(Fill)
        .secure(true)
        .style(|theme, _status| input_style(theme))
        .padding(10);

    let password_confirm_input = text_input("Confirm Password", &state.password_confirm)
        .on_input(Message::PasswordConfirmInputChanged)
        .on_submit(Message::RegisterButtonPressed)
        .line_height(1.5)
        .width(Fill)
        .secure(true)
        .style(|theme, _status| input_style(theme))
        .padding(10);

    let register_button = button(
        text(if state.is_loading {
            "Registering…"
        } else {
            "Register"
        })
        .width(Fill)
        .center(),
    )
    .on_press(Message::RegisterButtonPressed)
    .height(44)
    .width(Fill)
    .style(button::primary)
    .padding(10);

    let login_link = button(
        text("Login")
            .color(Color::from(AppColorMain::Primary))
            .size(14),
    )
    .on_press(Message::LoginInstead)
    .style(button::text)
    .height(14)
    .padding(0);

    container(
        column![
            text("Create your account")
                .font(RUBIK)
                .size(30)
                .width(Fill)
                .center(),
            column![
                column![
                    row![
                        text("USERNAME").size(12).font(RUBIK),
                        if let Some(err) = &state.username_error {
                            text("* ".to_string() + err.as_str())
                                .color(Color::from(AppColorStatus::Failure))
                                .size(14)
                        } else {
                            text("*")
                                .color(Color::from(AppColorStatus::Failure))
                                .size(14)
                        }
                    ]
                    .spacing(4),
                    username_input
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
                    row![
                        text("CONFIRM PASSWORD").size(12),
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
                    password_confirm_input
                ]
                .spacing(4),
            ]
            .spacing(24),
            column![
                register_button,
                row![
                    text("Have an account already? ")
                        .size(14)
                        .color(Color::from(AppColorForeground::Tertiary)),
                    login_link
                ],
            ]
            .spacing(12)
        ]
        .width(Length::Fixed(550.0))
        .padding(28)
        .spacing(36),
    )
    .style(|theme| container_style(theme))
    .into()
}

pub async fn perform_registration(
    username: String,
    password: String,
    password_confirm: String,
) -> Result<PublicUser, String> {
    println!(
        "Attempting registration for new user with username: {}.",
        username
    );

    if password != password_confirm {
        return Err("Passwords need to match".to_string());
    }

    let name = username.trim_ascii().to_string();

    let mut map = HashMap::new();
    map.insert("username", name);
    map.insert("password", password.clone());

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/register")
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
            state.username = value;
            state.username_error = None;
            Task::none()
        }
        Message::PasswordInputChanged(value) => {
            state.password = value;
            state.password_error = None;
            Task::none()
        }
        Message::PasswordConfirmInputChanged(value) => {
            state.password_confirm = value;
            state.password_error = None;
            Task::none()
        }
        Message::RegisterButtonPressed => {
            state.is_loading = true;
            let username = state.username.clone();
            let password = state.password.clone();
            let password_confirm = state.password_confirm.clone();
            Task::perform(
                perform_registration(username, password, password_confirm),
                |result| match result {
                    Ok(user) => Message::RegisterSuccess(user),
                    Err(e) => Message::RegisterFailed(e),
                },
            )
        }
        Message::RegisterSuccess(_) => {
            state.is_loading = false;
            Task::none()
        }
        Message::RegisterFailed(error) => {
            state.is_loading = false;
            state.username_error = Some(error.clone());
            state.password_error = Some(error);
            Task::none()
        }
        Message::LoginInstead => {
            state.is_loading = false;
            Task::done(Message::RequestScreenChange(super::CurrentScreen::Login(
                super::login_screen::State::new(),
            )))
        }
        _ => Task::none(),
    }
}
