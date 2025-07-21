use clap::{Parser, Subcommand};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Stylize,
    text::ToSpan,
    DefaultTerminal
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use std::{
    env,
    io,
};

include!(concat!(env!("OUT_DIR"), "/mac_vendor.rs"));

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Looks up a MAC address
    Lookup {
        /// The MAC address to look up.
        mac: Option<String>,
    },
    /// Forces an update of the OUI database
    Update,
}

fn lookup_vendor(mac: &str) -> Option<(&str, &str, &str)> {
    let prefix = (&mac[..8].to_uppercase()).replace(":", "");
    println!("{} ", prefix);
    MAC_VENDORS.get(prefix.as_str()).copied()
}

/// App holds the state of the application
#[derive(Debug, Default)]
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => self.start_editing(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => self.push_message(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                }
            }
        }
    }

    fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing
    }

    fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal
    }

    fn push_message(&mut self) {
        let mac_address = self.input.value_and_reset();
        if let Some((name, address, country)) = lookup_vendor(&mac_address) {
            self.messages.push(format!(
                "Vendor: {}, Address: {}, Country: {}",
                name, address, country
            ));
        } else {
            self.messages.push(format!("Vendor not found for MAC: {}", mac_address));
        }
    }

    fn render(&self, frame: &mut ratatui::Frame) {
        let [header_area, input_area, messages_area] = ratatui::layout::Layout::vertical([
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(3),
            ratatui::layout::Constraint::Min(1),
        ])
        .areas(frame.area());

        self.render_help_message(frame, header_area);
        self.render_input(frame, input_area);
        self.render_messages(frame, messages_area);
    }

    fn render_help_message(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let help_message = ratatui::text::Line::from_iter(match self.input_mode {
            InputMode::Normal => [
                "Press ".to_span(),
                "q".bold(),
                " to exit, ".to_span(),
                "e".bold(),
                " to start editing.".to_span(),
            ],
            InputMode::Editing => [
                "Press ".to_span(),
                "Esc".bold(),
                " to stop editing, ".to_span(),
                "Enter".bold(),
                " to record the message".to_span(),
            ],
        });
        frame.render_widget(help_message, area);
    }

    fn render_input(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => ratatui::style::Style::default(),
            InputMode::Editing => ratatui::style::Color::Yellow.into(),
        };
        let input = ratatui::widgets::Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(ratatui::widgets::Block::bordered().title("Input"));
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_messages(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let messages = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, message)| format!("{}: {}", i, message));
        let messages = ratatui::widgets::List::new(messages).block(ratatui::widgets::Block::bordered().title("Messages"));
        frame.render_widget(messages, area);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lookup { mac } => {
            if let Some(mac_address) = mac {
                if let Some((name, address, country)) = lookup_vendor(mac_address) {
                    println!(
                        "Vendor: {}, Address: {}, Country: {}",
                        name, address, country
                    );
                } else {
                    println!("Vendor not found");
                }
            } else {
                let mut terminal = ratatui::init();
                let result = App::default().run(&mut terminal);
                ratatui::restore();
                result?;
            }
        }
        Commands::Update => {
            println!("Updating OUI database...");
            //TODO: Implement update logic
            println!("OUI database updated successfully.");
        }
    }
    Ok(())
}
