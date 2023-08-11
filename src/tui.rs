use crate::api::{
    types::{Skills, Timestamp},
    Client,
};
use chrono::{TimeZone, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use num_format::{Locale, ToFormattedString};
use ratatui::{
    prelude::*,
    widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::{collections::BTreeMap, io};

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl StatefulList {
    pub fn with_items(items: Vec<String>) -> Self {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub client: Client,
    pub dataset: BTreeMap<Timestamp, Skills>,
    pub skills: StatefulList,
    pub cursor_position: usize,
    pub input: String,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(username: String) -> Self {
        let client = Client::new();
        let dataset = client.player_datapoints(&username, 1_000_000_000).unwrap();
        Self {
            client,
            dataset,
            skills: StatefulList::with_items(
                [
                    "Overall",
                    "Attack",
                    "Defence",
                    "Strength",
                    "Hitpoints",
                    "Ranged",
                    "Prayer",
                    "Magic",
                    "Cooking",
                    "Woodcutting",
                    "Fletching",
                    "Fishing",
                    "Firemaking",
                    "Crafting",
                    "Smithing",
                    "Mining",
                    "Herblore",
                    "Agility",
                    "Thieving",
                    "Slayer",
                    "Farming",
                    "Runecraft",
                    "Hunter",
                    "Construction",
                    // "Ehp",
                ]
                .into_iter()
                .map(std::borrow::ToOwned::to_owned)
                .collect(),
            ),
            cursor_position: username.len(),
            input: username,
            input_mode: InputMode::Normal,
        }
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_lossless,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn get_data(&self) -> Option<Vec<(f64, f64)>> {
        let selected = self.skills.state.selected()?;
        Some(
            self.dataset
                .iter()
                .map(|(k, v)| {
                    (
                        k.0.timestamp() as f64,
                        match selected {
                            0 => v.overall,
                            1 => v.attack as u64,
                            2 => v.defence as u64,
                            3 => v.strength as u64,
                            4 => v.hitpoints as u64,
                            5 => v.ranged as u64,
                            6 => v.prayer as u64,
                            7 => v.magic as u64,
                            8 => v.cooking as u64,
                            9 => v.woodcutting as u64,
                            10 => v.fletching as u64,
                            11 => v.fishing as u64,
                            12 => v.firemaking as u64,
                            13 => v.crafting as u64,
                            14 => v.smithing as u64,
                            15 => v.mining as u64,
                            16 => v.herblore as u64,
                            17 => v.agility as u64,
                            18 => v.thieving as u64,
                            19 => v.slayer as u64,
                            20 => v.farming as u64,
                            21 => v.runecraft as u64,
                            22 => v.hunter as u64,
                            23 => v.construction as u64,
                            24 => v.ehp as u64,
                            _ => unreachable!(),
                        } as f64,
                    )
                })
                .collect(),
        )
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn submit_username(&mut self) {
        self.dataset = self
            .client
            .player_datapoints(&self.input, 1_000_000_000)
            .unwrap();
        self.input_mode = InputMode::Normal;
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match app.input_mode {
                InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => app.skills.next(),
                        KeyCode::Up => app.skills.previous(),
                        KeyCode::Esc => app.skills.unselect(),
                        KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                        _ => {}
                    };
                }
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.submit_username(),
                    KeyCode::Char(to_insert) => app.enter_char(to_insert),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Esc => app.input_mode = InputMode::Normal,

                    _ => {}
                },
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to start editing.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to submit username".into(),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Username"));
    f.render_widget(input, chunks[1]);

    let chunks2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(15), Constraint::Percentage(75)].as_ref())
        .split(chunks[2]);

    let items: Vec<ListItem> = app
        .skills
        .items
        .iter()
        .map(|i| ListItem::new(i.clone()))
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Skill"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );
    // .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks2[0], &mut app.skills.state);

    let Some(experience) = app.get_data() else {
        return;
    };

    let dataset = Dataset::default()
        .name(&app.skills.items[app.skills.state.selected().unwrap()])
        .marker(symbols::Marker::Braille)
        .graph_type(ratatui::widgets::GraphType::Line)
        .style(Style::default().fg(Color::White))
        .data(&experience);

    let hunter = app
        .dataset
        .iter()
        .map(|(k, v)| (k.0.timestamp() as f64, v.hunter as f64))
        .collect::<Vec<_>>();
    let dataset2 = Dataset::default()
        .name("Hunter")
        .marker(symbols::Marker::Braille)
        .graph_type(ratatui::widgets::GraphType::Line)
        .style(Style::default().fg(Color::Green))
        .data(&hunter);

    let start_date = Utc
        .timestamp_opt(experience.first().unwrap().0 as i64, 0)
        .unwrap();
    let end_date = Utc
        .timestamp_opt(experience.last().unwrap().0 as i64, 0)
        .unwrap();
    let time_difference = end_date - start_date;
    let mid_point = start_date + time_difference / 2;

    let chart = Chart::new(vec![dataset, dataset2])
        .block(Block::default().borders(Borders::ALL).title("Experience"))
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds([experience.first().unwrap().0, experience.last().unwrap().0])
                .labels(vec![
                    format!("{}", start_date.format("%Y-%m-%d")).into(),
                    format!("{}", mid_point.format("%Y-%m-%d")).into(),
                    format!("{}", end_date.format("%Y-%m-%d")).into(),
                ])
                .labels_alignment(Alignment::Right),
        )
        .y_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds([experience.first().unwrap().1, experience.last().unwrap().1])
                .labels(vec![
                    format!("{:>13}", 0).into(),
                    format!(
                        "{:>13}",
                        (experience.last().unwrap().1 as u64).to_formatted_string(&Locale::en)
                    )
                    .into(),
                ]),
        );

    f.render_widget(chart, chunks2[1]);
}
