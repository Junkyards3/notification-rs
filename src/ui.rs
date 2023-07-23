use crate::app::{InputField, InputMode};
use crate::db::NotificationContent;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Row, Table};
use tui::Frame;

pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    notifications: &[NotificationContent],
    notifications_state: &mut ListState,
    last_update: DateTime<Local>,
    input_mode: InputMode,
    input_field: InputField,
    input_values: &HashMap<InputField, String>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let middle_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    draw_sync_date(f, last_update, chunks[0]);
    draw_keybindings(f, chunks[2], input_mode);
    draw_notification_list(
        f,
        notifications,
        notifications_state,
        middle_area[0],
        input_mode == InputMode::Navigation,
    );
    draw_notification_add(
        f,
        middle_area[1],
        input_mode == InputMode::Add,
        input_field,
        input_values,
    );
}

fn draw_notification_add<B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    is_active: bool,
    input_field: InputField,
    input_values: &HashMap<InputField, String>,
) {
    let block_color = match is_active {
        true => Color::LightCyan,
        false => Color::DarkGray,
    };

    let block = Block::default()
        .title("Add notification")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(block_color));

    let field_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);

    let selected_field_style = Style::default()
        .fg(Color::Black)
        .bg(Color::White)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);

    let table = Table::new(vec![
        Row::new(vec![
            Span::styled(
                "Title:",
                if input_field == InputField::Title {
                    selected_field_style
                } else {
                    field_style
                },
            ),
            Span::raw(
                input_values
                    .get(&InputField::Title)
                    .expect("title field should be present"),
            ),
        ])
        .bottom_margin(1),
        Row::new(vec![
            Span::styled(
                "Body:",
                if input_field == InputField::Body {
                    selected_field_style
                } else {
                    field_style
                },
            ),
            Span::raw(
                input_values
                    .get(&InputField::Body)
                    .expect("body field should be present"),
            ),
        ])
        .bottom_margin(1),
        Row::new(vec![
            Span::styled(
                "Date:",
                if input_field == InputField::Date {
                    selected_field_style
                } else {
                    field_style
                },
            ),
            Span::raw(
                input_values
                    .get(&InputField::Date)
                    .expect("date field should be present"),
            ),
        ])
        .bottom_margin(2),
        Row::new(vec![
            Span::raw("Date format:"),
            Span::raw("13:54 31/12/1970"),
        ]),
    ])
    .block(block)
    .widths(&[Constraint::Percentage(20), Constraint::Percentage(80)]);

    f.render_widget(table, area);
}

fn draw_notification_list<B: Backend>(
    f: &mut Frame<B>,
    notifications: &[NotificationContent],
    notifications_state: &mut ListState,
    area: Rect,
    is_active: bool,
) {
    let block_color = match is_active {
        true => Color::LightCyan,
        false => Color::DarkGray,
    };

    let list_items = notifications
        .iter()
        .map(|n| {
            ListItem::new(Spans::from(vec![
                Span::styled(&n.title, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::raw(&n.body),
                Span::raw(" "),
                Span::styled(
                    n.date.format("%d %B - %R").to_string(),
                    Style::default().fg(Color::Blue),
                ),
            ]))
        })
        .collect::<Vec<_>>();
    let list_items = List::new(list_items)
        .block(
            Block::default()
                .title("Notifications")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(block_color)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list_items, area, notifications_state);
}

fn draw_sync_date<B: Backend>(f: &mut Frame<B>, last_update: DateTime<Local>, area: Rect) {
    let block = Block::default()
        .title("Last synchronization")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(last_update.format("%X").to_string())
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);
}

fn draw_keybindings<B: Backend>(f: &mut Frame<B>, area: Rect, input_mode: InputMode) {
    let key_style = Style::default().bg(Color::Blue).fg(Color::White);

    let text_keys = match input_mode {
        InputMode::Navigation => Spans::from(vec![
            Span::styled("Navigate [↑↓]", key_style),
            Span::raw(" "),
            Span::styled("Delete [x]", key_style),
            Span::raw(" "),
            Span::styled("Add mode [a]", key_style),
            Span::raw(" "),
            Span::styled("Quit [q]", key_style),
            Span::raw(" "),
        ]),
        InputMode::Add => Spans::from(vec![
            Span::styled("Navigate [↑↓]", key_style),
            Span::raw(" "),
            Span::styled("Add [ENTER]", key_style),
            Span::raw(" "),
            Span::styled("Cancel [ESCAPE]", key_style),
            Span::raw(" "),
        ]),
    };

    let block_keys = Block::default();

    let keys_paragraph = Paragraph::new(text_keys)
        .block(block_keys)
        .alignment(Alignment::Left);
    f.render_widget(keys_paragraph, area);
}
