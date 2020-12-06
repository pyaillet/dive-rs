pub mod oci;
pub mod util;
use anyhow::Result;
use std::{env, error, io};

use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::Backend;
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

impl App {
    fn new(img_name: &str) -> Result<App, Box<dyn error::Error>> {
        let img_ref = img_name.parse::<oci::image::ImageReference>()?;

        let token = oci::registry::get_token(&img_ref)?;

        let manifest = oci::registry::get_manifest(&img_ref, &token)?;

        let config = oci::registry::get_config(&img_ref, &token)?;

        Ok(App {
            img_ref,
            manifest: Some(manifest),
            config: Some(config),
        })
    }
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()> {
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
    Ok(())
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

    let stdout = io::stdout().into_raw_mode()?;

    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    let app = App::new(&args[1])?;

    loop {
        draw(&mut terminal, &app)?;

        let Event::Input(input) = events.next()?;
        if let Key::Char('q') = input {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;
    use std::fs;

    #[test]
    fn test_config_to_vec() -> Result<(), Box<dyn error::Error>> {
        let config_content = fs::read_to_string("tests/resources/tests_local_config.json")?;
        let c: oci::image::config::Config = serde_json::from_str(&config_content)?;

        let mut v: Vec<ListItem> = Vec::new();
        v.push(ListItem::new(Span::from(
            "Created: 2020-10-22T02:19:24.499382102Z",
        )));
        v.push(ListItem::new(Span::from("Arch: Amd64")));
        v.push(ListItem::new(Span::from("OS: Linux")));
        v.push(ListItem::new(Span::from("Configuration")));
        v.push(ListItem::new(Span::from("> User: ")));

        assert_eq!(config_to_vec(&Some(c)), v);
        Ok(())
    }

    #[test]
    fn test_manifest_to_vec() -> Result<(), Box<dyn error::Error>> {
        let manifest_content = fs::read_to_string("tests/resources/tests_local_manifest.json")?;
        let m: oci::image::manifest::Manifest = serde_json::from_str(&manifest_content)?;

        let mut v: Vec<ListItem> = Vec::new();
        let item = ListItem::new(Span::from(
            "> sha256:188c0c94c7c576fff0792aca7ec73d67a2f7f4cb3a6e53a84559337260b36964",
        ));
        v.push(item);
        assert_eq!(manifest_to_vec(&Some(m)), v);
        Ok(())
    }
}
