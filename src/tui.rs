use std::{collections::BTreeMap, io};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem, ListState},
    Frame, Terminal,
};

use crate::api::types::{Skills, Timestamp};

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

pub struct App {
    pub dataset: BTreeMap<Timestamp, Skills>,
    pub skills: StatefulList,
}

impl App {
    pub fn new(dataset: BTreeMap<Timestamp, Skills>) -> Self {
        Self {
            dataset,
            skills: StatefulList::with_items(vec![
                "Overall".to_owned(),
                "Attack".to_owned(),
                "Defence".to_owned(),
                "Strength".to_owned(),
                "Hitpoints".to_owned(),
                "Ranged".to_owned(),
                "Prayer".to_owned(),
                "Magic".to_owned(),
                "Cooking".to_owned(),
                "Woodcutting".to_owned(),
                "Fletching".to_owned(),
                "Fishing".to_owned(),
                "Firemaking".to_owned(),
                "Crafting".to_owned(),
                "Smithing".to_owned(),
                "Mining".to_owned(),
                "Herblore".to_owned(),
                "Agility".to_owned(),
                "Thieving".to_owned(),
                "Slayer".to_owned(),
                "Farming".to_owned(),
                "Runecraft".to_owned(),
                "Hunter".to_owned(),
                "Construction".to_owned(),
                "Ehp".to_owned(),
            ]),
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
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.skills.next(),
                    KeyCode::Up => app.skills.previous(),
                    KeyCode::Esc => app.skills.unselect(),
                    _ => {}
                };
            }
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

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
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[0], &mut app.skills.state);

    let Some(overall) = app.get_data() else {
        return;
    };

    let dataset = Dataset::default()
        .name(&app.skills.items[app.skills.state.selected().unwrap()])
        .marker(symbols::Marker::Braille)
        .graph_type(ratatui::widgets::GraphType::Line)
        .style(Style::default().fg(Color::White))
        .data(&overall);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().borders(Borders::ALL).title("Overall"))
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds([overall.first().unwrap().0, overall.last().unwrap().0]),
        )
        .y_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds([overall.first().unwrap().1, overall.last().unwrap().1])
                .labels(vec![
                    "0".into(),
                    format!("{}", overall.last().unwrap().1).into(),
                ]),
        );

    f.render_widget(chart, chunks[1]);
}
