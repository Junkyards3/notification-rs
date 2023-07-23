use ::chrono::{NaiveDateTime, ParseError};
use crossterm::event::KeyCode;
use std::collections::HashMap;
use tokio::task::JoinHandle;
use tui::widgets::ListState;

use crate::db::NotificationContent;
use crate::notification::prepare_notification_sending_task;

pub struct NotificationTask {
    pub content: NotificationContent,
    pub handle: JoinHandle<()>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InputMode {
    Navigation,
    Add,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InputField {
    Title,
    Body,
    Date,
}

impl InputField {
    pub fn next(self) -> InputField {
        match self {
            InputField::Title => InputField::Body,
            InputField::Body => InputField::Date,
            InputField::Date => InputField::Title,
        }
    }

    pub fn previous(self) -> InputField {
        match self {
            InputField::Title => InputField::Date,
            InputField::Body => InputField::Title,
            InputField::Date => InputField::Body,
        }
    }
}

pub struct App {
    pub state: ListState,
    pub notifications: Vec<NotificationTask>,
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub input_field: InputField,
    pub input_values: HashMap<InputField, String>,
}

impl App {
    pub fn new(notifications: Vec<NotificationContent>) -> App {
        App {
            notifications: notifications
                .into_iter()
                .filter_map(|notification| prepare_notification_sending_task(notification).ok())
                .collect(),
            state: ListState::default(),
            should_quit: false,
            input_mode: InputMode::Navigation,
            input_field: InputField::Title,
            input_values: HashMap::from([
                (InputField::Title, String::new()),
                (InputField::Body, String::new()),
                (InputField::Date, String::new()),
            ]),
        }
    }

    pub fn get_notifications_content(&self) -> Vec<NotificationContent> {
        self.notifications
            .iter()
            .map(|n| n.content.clone())
            .collect()
    }

    fn delete_selected_notification(&mut self) {
        if let Some(i) = self.state.selected() {
            let notification_to_delete = self.notifications.remove(i);
            notification_to_delete.handle.abort();
            if self.notifications.is_empty() {
                self.state.select(None);
            } else {
                let index_to_select = if i == self.notifications.len() {
                    i - 1
                } else {
                    i
                };
                self.state.select(Some(index_to_select));
            }
        }
    }

    fn add_notification(&mut self) -> Result<(), ParseError> {
        let date = NaiveDateTime::parse_from_str(
            self.input_values.get(&InputField::Date).unwrap(),
            "%H:%M %d/%m/%Y",
        )?;
        let notification = NotificationContent {
            title: self
                .input_values
                .get(&InputField::Title)
                .unwrap()
                .to_string(),
            body: self
                .input_values
                .get(&InputField::Body)
                .unwrap()
                .to_string(),
            date,
        };

        let notification_task = prepare_notification_sending_task(notification);
        if let Ok(notification_task) = notification_task {
            self.notifications.push(notification_task);
            self.input_values.iter_mut().for_each(|(_, v)| v.clear());
        }
        Ok(())
    }

    pub fn clean_sent_notifications(&mut self) {
        self.notifications
            .retain(|notification| !notification.handle.is_finished());
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match (self.input_mode, key) {
            (InputMode::Navigation, KeyCode::Up) => self.on_up(),
            (InputMode::Navigation, KeyCode::Down) => self.on_down(),
            (InputMode::Navigation, KeyCode::Char('x')) => self.delete_selected_notification(),
            (InputMode::Navigation, KeyCode::Char('a')) => {
                self.input_mode = InputMode::Add;
            }
            (InputMode::Navigation, KeyCode::Char('q')) => self.should_quit = true,
            (InputMode::Add, KeyCode::Up) => {
                self.input_field = self.input_field.previous();
            }
            (InputMode::Add, KeyCode::Down) => {
                self.input_field = self.input_field.next();
            }
            (InputMode::Add, KeyCode::Enter) => {
                self.add_notification().unwrap_or_default();
                self.input_mode = InputMode::Navigation;
            }
            (InputMode::Add, KeyCode::Esc) => {
                self.input_mode = InputMode::Navigation;
            }
            (InputMode::Add, KeyCode::Char(c)) => {
                let input_value = self.input_values.get_mut(&self.input_field).unwrap();
                input_value.push(c);
            }
            (InputMode::Add, KeyCode::Backspace) => {
                let input_value = self.input_values.get_mut(&self.input_field).unwrap();
                input_value.pop();
            }
            _ => {}
        }
    }

    fn on_up(&mut self) {
        if self.notifications.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    self.notifications.len() - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn on_down(&mut self) {
        if self.notifications.is_empty() {
            return;
        }
        if let Some(i) = self.state.selected() {
            if i >= self.notifications.len() - 1 {
                self.state.select(Some(0));
            } else {
                self.state.select(Some(i + 1));
            }
        } else {
            self.state.select(Some(0));
        }
    }
}
