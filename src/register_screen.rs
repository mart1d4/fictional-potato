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
use validator::Validate;
use zxcvbn::zxcvbn;

#[derive(Debug, Clone, Default, Validate)]
pub struct State {
    #[validate(length(min = 2, max = 32, message = "Must be between 2 to 32 characters"))]
    pub username: String,
    #[validate(length(min = 1, max = 72, message = "Must be between 2 to 72 characters"))]
    pub password: String,
    #[validate(length(min = 1, max = 72, message = "Must be between 2 to 72 characters"))]
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
    .style(button_style)
    .padding(10);

    let login_link = button(
        text("Login")
            .color(Color::from(AppColorMain::Secondary))
            .size(12),
    )
    .on_press(Message::LoginInstead)
    .style(button::text)
    .height(16)
    .padding(0);

    container(
        column![
            text("Create your account")
                .font(RUBIK)
                .size(24)
                .width(Fill)
                .center(),
            column![
                styled_input(
                    "Username",
                    &state.username,
                    state.username_error.as_deref(),
                    None,
                    Message::UsernameInputChanged,
                    //Message::RegisterButtonPressed,
                    None,
                    Some(true),
                ),
                styled_input(
                    "Password",
                    &state.password,
                    state.password_error.as_deref(),
                    None,
                    Message::PasswordInputChanged,
                    //Message::RegisterButtonPressed,
                    None,
                    Some(true),
                ),
                styled_input(
                    "Confirm Password",
                    &state.password_confirm,
                    state.password_error.as_deref(),
                    None,
                    Message::PasswordConfirmInputChanged,
                    //Message::RegisterButtonPressed,
                    Some(true),
                    Some(true),
                ),
            ]
            .spacing(24),
            column![
                register_button,
                row![
                    text("Have an account already? ")
                        .size(12)
                        .color(Color::from(AppColorForeground::SubtextPrimary)),
                    login_link
                ],
            ]
            .spacing(8)
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
) -> Result<PublicUser, String> {
    println!(
        "Attempting registration for new user with username: {}.",
        username
    );

    let name = username.trim_ascii().to_string();

    let mut map = HashMap::new();
    map.insert("username", name);
    map.insert("password", password);

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8585/auth/register")
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
        set_token_from_secure_storage(Some(success_body.refresh_token)).map_err(|e| "Err")?;
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
            let estimate = zxcvbn(&value, &[]);
            println!("Estimate: {}", estimate.score());
            println!("Guesses: {}", estimate.guesses());
            println!("Crack time: {:?}", estimate.crack_times());
            println!("Feedback: {:?}\n", estimate.feedback());
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
            match state.validate() {
                Ok(_) => {}
                Err(e) => {
                    state.is_loading = false;
                    if let Some(username_errors) = e.field_errors().get("username") {
                        if let Some(first_error) = username_errors.first() {
                            if let Some(message) = &first_error.message {
                                state.username_error = Some(message.to_string());
                            }
                        }
                    }
                    if let Some(password_errors) = e.field_errors().get("password") {
                        if let Some(first_error) = password_errors.first() {
                            if let Some(message) = &first_error.message {
                                state.password_error = Some(message.to_string());
                            }
                        }
                    }
                    return Task::none();
                }
            }
            if state.password != state.password_confirm {
                state.password_error = Some("Passwords need to match.".to_string());
                return Task::none();
            }
            Task::perform(
                perform_registration(state.username.clone(), state.password.clone()),
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
