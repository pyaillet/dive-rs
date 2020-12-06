pub mod oci;
pub mod util;
use anyhow::Result;
use std::{env, error, io};

use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use oci::image::ImageReference;
use util::event::{Event, Events};

struct App {
    img_ref: ImageReference,
    manifest: Option<oci::image::manifest::Manifest>,
    config: Option<oci::image::config::Config>,
}

fn config_to_vec(config: &Option<oci::image::config::Config>) -> Vec<ListItem> {
    match config {
        None => Vec::new(),
        Some(c) => {
            let mut v = Vec::new();
            if let Some(author) = &c.author {
                v.push(ListItem::new(Span::from(format!("Author: {}", author))));
            }
            v.push(ListItem::new(Span::from(format!("Created: {}", c.created))));
            v.push(ListItem::new(Span::from(format!(
                "Arch: {}",
                c.architecture
            ))));
            v.push(ListItem::new(Span::from(format!("OS: {}", c.os))));
            v.push(ListItem::new(Span::from("Configuration")));
            if let Some(user) = &c.config.user {
                v.push(ListItem::new(Span::from(format!("> User: {}", user))));
            }
            v
        }
    }
}

fn manifest_to_vec(manifest: &Option<oci::image::manifest::Manifest>) -> Vec<ListItem> {
    match manifest {
        None => Vec::new(),
        Some(m) => m
            .layers
            .iter()
            .map(|l| ListItem::new(Span::from(format!("> {}", l.digest))))
            .collect::<Vec<ListItem>>(),
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let img_ref = args[1]
        .parse::<oci::image::ImageReference>()
        .expect("Unable to parse image ref");

    let stdout = io::stdout().into_raw_mode()?;

    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    let token = oci::registry::get_token(&img_ref)?;

    let manifest = oci::registry::get_manifest(&img_ref, &token)?;

    let config = oci::registry::get_config(&img_ref, &token)?;

    let app = App {
        img_ref,
        manifest: Some(manifest),
        config: Some(config),
    };

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(20),
                        Constraint::Min(20),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let paragraph = Paragraph::new("Dive-rs");
            f.render_widget(paragraph, chunks[0]);
            let block = Block::default().title("Manifest").borders(Borders::ALL);
            let list = List::new(config_to_vec(&app.config)).block(block);
            f.render_widget(list, chunks[1]);
            let block = Block::default().title("Layers").borders(Borders::ALL);
            let list = List::new(manifest_to_vec(&app.manifest)).block(block);
            f.render_widget(list, chunks[2]);
            let block = Block::default().title("Status").borders(Borders::ALL);
            let paragraph = Paragraph::new(format!("Loaded image: {}", app.img_ref)).block(block);
            f.render_widget(paragraph, chunks[3]);
        })?;

        let Event::Input(input) = events.next()?;
        if let Key::Char('q') = input {
            break;
        }
    }

    Ok(())
}
