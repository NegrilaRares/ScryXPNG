use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::ui::app_data::Screen;

use super::App;

impl App {
    pub fn handle_events(&mut self) {
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event.code);
                }
                _ => {}
            }
        };
    }

    pub fn handle_key_event(&mut self, event: KeyCode) {
        match event {
            KeyCode::Char('q') => {
                self.exit();
            }
            KeyCode::Char('h') => {
                self.toggle_help();
            }
            KeyCode::Enter => {
                self.change_to_list();
                self.submit_message();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Char('a') => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            KeyCode::Char('d') => {
                self.move_cursor_right();
            }
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Char(to_insert) => self.enter_char(to_insert),

            _ => {}
        }
    }

    pub fn exit(&mut self) {
        self.context.exit = true;
    }

    pub fn toggle_help(&mut self) {
        match self.status {
            Screen::Screen0 => self.status = Screen::Screen1,
            Screen::Screen1 => self.status = Screen::Screen0,
            Screen::Screen2 => self.status = Screen::Screen3,
            Screen::Screen3 => self.status = Screen::Screen2,
            Screen::Screen4 => self.status = Screen::Screen5,
            Screen::Screen5 => self.status = Screen::Screen4,
        }
    }

    pub fn change_to_list(&mut self)
    {
        if self.status == Screen::Screen0 {
            self.status = Screen::Screen2;
        }
        else if self.status == Screen::Screen1 {
            self.status = Screen::Screen3;
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.context.selected_index.saturating_sub(1);
        self.context.selected_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.context.selected_index.saturating_add(1);
        self.context.selected_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.context.destination.chars().count())
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.context.destination.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.context
            .destination
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.context.selected_index)
            .unwrap_or(self.context.destination.len())
    }

    pub fn reset_cursor(&mut self) {
        self.context.selected_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.context.destination.clear();
        self.reset_cursor();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.context.selected_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.context.selected_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self
                .context
                .destination
                .chars()
                .take(from_left_to_current_index);
            let after_char_to_delete = self.context.destination.chars().skip(current_index);

            self.context.destination = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
}
