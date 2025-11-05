mod colors;
mod components;
mod constants;
mod login_screen;
mod register_screen;
mod screens;
mod styles;
mod utils;

use chrono::{DateTime, Utc};
use iced::event::{self, Event};
use iced::keyboard::key;
use iced::widget::{self, button, container, text};
use iced::{Element, Fill, Font, Subscription, Task, Theme, keyboard};
use turbo::types::PublicUser;

use crate::colors::AppColorMain;
use crate::screens::app_screen;
use crate::styles::button_style;
use crate::utils::get_user_with_token;

pub fn main() -> iced::Result {
    iced::application("Fictional Potato", App::update, App::view)
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
        .run_with(App::new)
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
    App(app_screen::State),
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal(ModalType),
    HideModal,
    ShowDialog(DialogType),
    HideDialog,
    Event(Event),

    RefreshTokenChecked(Result<PublicUser, String>),

    ChangeCurrentScreen(CurrentScreen),
    LogUserIn(Option<PublicUser>),

    LoginScreenMessage(login_screen::Message),
    RegisterScreenMessage(register_screen::Message),
    AppScreenMessage(app_screen::Message),
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

    fn new() -> (Self, Task<Message>) {
        (
            App::default(),
            Task::perform(get_user_with_token(), Message::RefreshTokenChecked),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeCurrentScreen(screen) => {
                self.current_screen = screen;
                Task::none()
            }
            Message::LoginScreenMessage(msg) => {
                if let CurrentScreen::Login(login_state) = &mut self.current_screen {
                    let command = login_screen::update(login_state, msg);
                    command.map(|sub_msg| match sub_msg {
                        login_screen::Message::LoginSuccess(user) => Message::LogUserIn(Some(user)),
                        login_screen::Message::RequestScreenChange(screen) => {
                            Message::ChangeCurrentScreen(screen)
                        }
                        _ => Message::LoginScreenMessage(sub_msg),
                    })
                } else {
                    Task::none()
                }
            }
            Message::RegisterScreenMessage(msg) => {
                if let CurrentScreen::Register(register_state) = &mut self.current_screen {
                    let command = register_screen::update(register_state, msg);
                    command.map(|sub_msg| match sub_msg {
                        register_screen::Message::RegisterSuccess(user) => {
                            Message::LogUserIn(Some(user))
                        }
                        register_screen::Message::RequestScreenChange(screen) => {
                            Message::ChangeCurrentScreen(screen)
                        }
                        _ => Message::RegisterScreenMessage(sub_msg),
                    })
                } else {
                    Task::none()
                }
            }
            Message::LogUserIn(user_data) => {
                self.user = user_data;
                self.current_screen = CurrentScreen::App(app_screen::State::new());
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
            Message::RefreshTokenChecked(res) => {
                if res.is_err() {
                    println!(
                        "Error getting user from token: {}",
                        res.err().unwrap_or_default()
                    );
                    Task::done(Message::ChangeCurrentScreen(CurrentScreen::Login(
                        login_screen::State::new(),
                    )))
                } else {
                    let user = res.ok().unwrap();
                    self.user = Some(user);
                    Task::done(Message::ChangeCurrentScreen(CurrentScreen::App(
                        app_screen::State::new(),
                    )))
                }
            }
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
            CurrentScreen::Loading => button(text("Go to register screen").center())
                .height(40)
                .padding(10)
                .style(button_style)
                .on_press(Message::ChangeCurrentScreen(CurrentScreen::Register(
                    register_screen::State::new(),
                )))
                .into(),
            CurrentScreen::App(state) => app_screen::view(&state).map(Message::AppScreenMessage),
        };

        container(content)
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .style(|_t| container::Style {
                background: Some(AppColorMain::Primary.to_bg()),
                ..Default::default()
            })
            .into()
    }
}
