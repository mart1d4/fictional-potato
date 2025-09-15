mod login_screen;

use chrono::{DateTime, Utc};
use iced::event::{self, Event};
use iced::keyboard::key;
use iced::widget::{self, button, center, column, container, mouse_area, opaque, stack, text};
use iced::{Color, Element, Fill, Subscription, Task, Theme, keyboard};

pub fn main() -> iced::Result {
    iced::application("My title", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Debug, Default)]
pub struct User {
    pub id: i64,
}

#[derive(Debug, Default)]
pub struct App {
    pub user: Option<User>,
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
    Register,
    Login(login_screen::State),
    App,
}

#[derive(Debug, Clone)]
enum Message {
    ShowModal(ModalType),
    HideModal,
    ShowDialog(DialogType),
    HideDialog,
    ChangeScreen(CurrentScreen),
    Event(Event),

    ChangeLoginScreen,

    LoginScreenMessage(login_screen::Message),
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
    fn title(&self) -> String {
        format!("Fictional Potato")
    }

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeLoginScreen => {
                if let CurrentScreen::Loading = &mut self.current_screen {
                    self.current_screen = CurrentScreen::Login(login_screen::State::new());
                } else {
                    self.current_screen = CurrentScreen::Loading;
                }
                Task::none()
            }
            Message::LoginScreenMessage(msg) => {
                if let CurrentScreen::Login(login_state) = &mut self.current_screen {
                    // Update the sub-screen's state
                    let command = login_screen::update(login_state, msg);

                    // If the sub-screen returns a command, map its message to our App's message
                    command.map(|sub_msg| {
                        match sub_msg {
                            // Intercept the LoginSuccess message from the LoginScreen
                            // to trigger a top-level transition
                            login_screen::Message::LoginSuccess => Message::ChangeLoginScreen,
                            // Other messages from LoginScreen don't trigger top-level transitions
                            _ => Message::LoginScreenMessage(sub_msg),
                        }
                    })
                } else {
                    Task::none()
                }
            }
            //Message::ChangeScreen(new_screen) => {
            //    self.current_screen = new_screen;
            //    Task::none()
            //}
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
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => Task::none(),
                _ => Task::none(),
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match &self.current_screen {
            CurrentScreen::Login(state) => {
                // Render the LoginScreen's view and map its messages
                login_screen::view(&state).map(Message::LoginScreenMessage)
            }
            CurrentScreen::Register => text("Register Screen").into(),
            //ScreenType::MainApp(state) => {
            //    // Render the MainAppScreen's view and map its messages
            //    main_app_screen::view(state).map(Message::MainAppScreenMessage)
            //}
            CurrentScreen::Loading => {
                let co = column![
                    text("Loading..."),
                    button(text("Go to login screen")).on_press(Message::ChangeLoginScreen)
                ]
                .into();

                return co;
            }
            _ => text("Not yet implemented!").into(),
        };

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}

//fn modal<'a, Message>(
//    base: impl Into<Element<'a, Message>>,
//    content: impl Into<Element<'a, Message>>,
//    on_blur: Message,
//) -> Element<'a, Message>
//where
//    Message: Clone + 'a,
//{
//    stack![
//        base.into(),
//        opaque(
//            mouse_area(center(opaque(content)).style(|_theme| {
//                container::Style {
//                    background: Some(
//                        Color {
//                            a: 0.8,
//                            ..Color::BLACK
//                        }
//                        .into(),
//                    ),
//                    ..container::Style::default()
//                }
//            }))
//            .on_press(on_blur)
//        )
//    ]
//    .into()
//}
