use chrono::format::Fixed;
use iced::{
    Element,
    Length::{self, Fill},
    Task,
    widget::{button, column, text, text_input},
};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub username: String,
    pub password: String,
    pub is_loading: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            is_loading: false,
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
}

pub fn view(state: &State) -> Element<'_, Message> {
    let username_input = text_input("Username", &state.username)
        .on_input(Message::UsernameInputChanged)
        .on_submit(Message::LoginButtonPressed)
        .width(Fill)
        .padding(10);

    let password_input = text_input("Password", &state.password)
        .on_input(Message::PasswordInputChanged)
        .on_submit(Message::LoginButtonPressed)
        .width(Fill)
        .padding(10);

    let login_button = button(text("Login").width(Fill).center())
        .on_press(Message::LoginButtonPressed)
        .height(44)
        .width(Fill)
        .style(button::success)
        .padding(10);

    column![
        text("Login Screen").size(30),
        username_input,
        password_input,
        login_button,
        if state.is_loading {
            text("Logging in...")
        } else {
            text("")
        }
    ]
    .width(Length::Fixed(550.0))
    .padding(24)
    .spacing(10)
    .into()
}

// In a real app, this would perform an actual async login request
pub async fn perform_login(username: String, password: String) -> Result<(), String> {
    println!("Attempting login for {}...", username);
    // Simulate network delay
    //let body = reqwest::get("https://www.rust-lang.org")
    //    .await
    //    .map_err(String::from("MAIS NON"))?
    //    .text()
    //    .await
    //    .map_err(String::from("MAIS NON"))?;

    //println!("body = {body:?}");
    if username == "user" && password == "pass" {
        Ok(())
    } else {
        Err("Invalid credentials".to_string())
    }
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::UsernameInputChanged(value) => {
            state.username = value;
            Task::none()
        }
        Message::PasswordInputChanged(value) => {
            state.password = value;
            Task::none()
        }
        Message::LoginButtonPressed => {
            state.is_loading = true;
            let username = state.username.clone();
            let password = state.password.clone();
            Task::perform(perform_login(username, password), |result| match result {
                Ok(_) => Message::LoginSuccess,
                Err(e) => Message::LoginFailed(e),
            })
        }
        Message::LoginSuccess => {
            state.is_loading = false;
            // This message needs to be mapped to the main App's Message
            Task::none() // This specific message needs to be handled by the parent
        }
        Message::LoginFailed(error) => {
            state.is_loading = false;
            eprintln!("Login failed: {}", error);
            Task::none()
        }
    }
}
