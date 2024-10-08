pub mod backend;
pub mod commands;
pub mod text_display;

use ratatui::DefaultTerminal;

use crate::ui::app_data::{context::Context, Screen};

pub struct App {
    pub status: Screen,
    pub context: Context,
}

impl App {
    pub fn new() -> App {
        App {
            status: Screen::Screen0,
            context: Context::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        terminal.hide_cursor().unwrap();
        while !self.context.exit {
            self.select_help_text();
            self.handle_events();
            terminal.draw(|frame| self.draw(frame)).unwrap();
        }
    }
}
