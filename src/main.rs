mod colors;
mod components;
mod constants;
mod login_screen;
mod register_screen;
mod styles;

use chrono::{DateTime, Utc};
use iced::event::{self, Event};
use iced::keyboard::key;
use iced::widget::{self, button, container, text};
use iced::{Element, Fill, Font, Subscription, Task, Theme, keyboard};
use turbo::types::PublicUser;

pub fn main() -> iced::Result {
    iced::application("My title", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        //       .font(include_bytes!("../fonts/static/Roboto-Black.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-Bold.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-ExtraBold.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-ExtraLight.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-Light.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-Medium.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-Regular.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-SemiBold.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-Thin.ttf").as_slice())
        //       .font(include_bytes!("../fonts/static/Roboto-BlackItalic.ttf").as_slice())
        .font(include_bytes!("../fonts/RubikMonoOne-Regular.ttf").as_slice())
        .default_font(RUBIK)
        .run()
}

//pub const ROBOTO_BLACK: Font = Font::with_name("Roboto-Black");
//pub const ROBOTO_BLACK_ITALIC: Font = Font::with_name("Roboto-BlackItalic");
//pub const ROBOTO_THIN: Font = Font::with_name("Roboto-Thin");
pub const RUBIK: Font = Font::with_name("RubikMonoOne-Regular");

#[derive(Debug, Default)]
pub struct App {
    pub user: Option<PublicUser>,
    pub theme: Option<Theme>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub current_modal: Option<ModalType>,
    pub current_dialog: Option<DialogType>,
    pub token_expires: DateTime<Utc>,
    pub current_screen: CurrentScreen,
}

#[derive(Debug, Clone, Default)]
pub enum CurrentScreen {
    #[default]
    Loading,
    Register(register_screen::State),
    Login(login_screen::State),
    App,
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal(ModalType),
    HideModal,
    ShowDialog(DialogType),
    HideDialog,
    Event(Event),

    ChangeCurrentScreen(CurrentScreen),
    RegisterAndLoggedIn(Option<PublicUser>),

    LoginScreenMessage(login_screen::Message),
    RegisterScreenMessage(register_screen::Message),
}

#[derive(Debug, Clone)]
pub enum ModalType {
    CreateDM,
    AddFriendsToGroupDM,
    CreateGuild,
    CreateChannel,
    ModifyUsername,
    ModifyPassword,
    ModifyEmail,
    ModifyPhone,
}

#[derive(Debug, Clone)]
pub enum DialogType {
    Logout,
    LogoutAllDevices,
    CallIncoming,
    PinnedMessages,
}

impl App {
    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeCurrentScreen(screen) => {
                println!("Changing current screen!");
                self.current_screen = screen;
                Task::none()
            }
            Message::LoginScreenMessage(msg) => {
                println!("Got login screen message");
                if let CurrentScreen::Login(login_state) = &mut self.current_screen {
                    // Update the sub-screen's state
                    let command = login_screen::update(login_state, msg);

                    // If the sub-screen returns a command, map its message to our App's message
                    command.map(|sub_msg| {
                        match sub_msg {
                            // Intercept the LoginSuccess message from the LoginScreen
                            // to trigger a top-level transition
                            login_screen::Message::LoginSuccess => {
                                Message::ChangeCurrentScreen(CurrentScreen::App)
                            }
                            login_screen::Message::RequestScreenChange(screen_to_change_to) => {
                                println!("App received explicit request to change screen!");
                                Message::ChangeCurrentScreen(screen_to_change_to)
                            }
                            // Other messages from LoginScreen don't trigger top-level transitions
                            _ => Message::LoginScreenMessage(sub_msg),
                        }
                    })
                } else {
                    Task::none()
                }
            }
            Message::RegisterScreenMessage(msg) => {
                println!("Got register screen message");
                if let CurrentScreen::Register(register_state) = &mut self.current_screen {
                    println!("Got register screen message | in if condition");
                    // Update the sub-screen's state
                    let command = register_screen::update(register_state, msg);

                    // If the sub-screen returns a command, map its message to our App's message
                    command.map(|sub_msg| {
                        match sub_msg {
                            // Intercept the LoginSuccess message from the LoginScreen
                            // to trigger a top-level transition
                            register_screen::Message::RegisterSuccess(user) => {
                                Message::RegisterAndLoggedIn(Some(user))
                            }
                            register_screen::Message::RequestScreenChange(screen_to_change_to) => {
                                println!("App received explicit request to change screen!");
                                Message::ChangeCurrentScreen(screen_to_change_to)
                            }
                            // Other messages from LoginScreen don't trigger top-level transitions
                            _ => Message::RegisterScreenMessage(sub_msg),
                        }
                    })
                } else {
                    Task::none()
                }
            }
            Message::RegisterAndLoggedIn(user_data) => {
                self.user = user_data;
                self.current_screen = CurrentScreen::App;
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match &self.current_screen {
            CurrentScreen::Login(state) => {
                login_screen::view(&state).map(Message::LoginScreenMessage)
            }
            CurrentScreen::Register(state) => {
                register_screen::view(&state).map(Message::RegisterScreenMessage)
            }
            CurrentScreen::Loading => {
                button(text("Go to login screen").font(RUBIK).width(180).center())
                    .height(40)
                    .on_press(Message::ChangeCurrentScreen(CurrentScreen::Register(
                        register_screen::State::new(),
                    )))
                    .into()
            }
            CurrentScreen::App => {
                let mut username = "USERISUNDEFINED".to_string();
                if self.user.is_some() {
                    username = self.user.clone().unwrap().username;
                }
                text(format!("Welcome to your account, {}!", username)).into()
            }
            _ => text("Not yet implemented!").into(),
        };

        container(content)
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .into()
    }
}
