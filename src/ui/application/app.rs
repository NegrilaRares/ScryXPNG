use ratatui::DefaultTerminal;

use crate::ui::app_data::{context::Context, status::{Screen0, States}};

pub struct App {
    pub status: States,
    pub context: Context,
}

impl App {
    pub fn new() -> App {
        App {
            status: States::Screen0(Screen0::new()),
            context: Context::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        terminal.hide_cursor();
        while !self.context.exit {
            self.handle_events();
            let _ = terminal.draw(|frame| {
                match &self.status {
                    States::Screen0(screen) => frame.render_widget(screen, frame.area()),
                    States::Screen1(screen) => frame.render_widget(screen, frame.area()),
                    States::Screen2(screen) => frame.render_widget(screen, frame.area()),
                    States::Screen3(screen) => frame.render_widget(screen, frame.area()),
                    States::Screen4(screen) => frame.render_widget(screen, frame.area()),
                    States::Screen5(screen) => frame.render_widget(screen, frame.area()),
                }
            }
        );
        }
    }

    
}