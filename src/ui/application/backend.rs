use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Stylize},
    text::Line,
};

use crate::ui::app_data::{
    context::{InputMode, SubWindow},
    Screen,
};

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
        match self.context.input_mode {
            InputMode::Normal => match event {
                KeyCode::Char('q') => {
                    self.context.prev_event_buf = Some(KeyCode::Char('q'));
                    self.exit();
                }
                KeyCode::Char('h') => {
                    self.context.prev_event_buf = Some(KeyCode::Char('h'));
                    self.toggle_help();
                }
                KeyCode::Char('e') => {
                    if self.context.prev_event_buf == Some(KeyCode::Tab)
                        && self.context.selected_subwindow.is_destination()
                    {
                        self.toggle_editing();
                    }
                    self.context.prev_event_buf = Some(KeyCode::Char('e'));
                }
                KeyCode::Enter => {
                    self.context.prev_event_buf = Some(KeyCode::Enter);
                    if self.context.selected_subwindow.is_destination() {
                        self.change_to_list();
                        self.set_destination(self.context.destination_temporary.clone());
                        self.clear_message();
                    }
                    if self.context.input_mode.is_editing() {
                        self.toggle_editing();
                    }
                }
                KeyCode::Tab => {
                    self.context.prev_event_buf = Some(KeyCode::Tab);
                }
                KeyCode::Char('1') => {
                    if self.context.prev_event_buf == Some(KeyCode::Tab) {
                        self.context.selected_subwindow = SubWindow::Destination;
                    }
                }
                KeyCode::Char('2') => {
                    if self.context.prev_event_buf == Some(KeyCode::Tab) {
                        if self.status.is_2() || self.status.is_3() || self.status.is_4() || self.status.is_5() {
                            self.context.selected_subwindow = SubWindow::List;
                        }
                    }
                }
                KeyCode::Char('3') => {
                    if self.context.prev_event_buf == Some(KeyCode::Tab) {
                        if self.status.is_4() || self.status.is_5() {
                            self.context.selected_subwindow = SubWindow::PickCard;
                        }
                    }
                }
                _ => {}
            },
            InputMode::Editing => match event {
                KeyCode::Tab => {
                    self.context.prev_event_buf = Some(KeyCode::Tab);
                }
                KeyCode::Enter => {
                    self.context.prev_event_buf = Some(KeyCode::Enter);
                    if self.context.selected_subwindow.is_destination() {
                        self.change_to_list();
                        self.set_destination(self.context.destination_temporary.clone());
                        self.clear_message();

                    }
                    if self.context.input_mode.is_editing() {
                        self.toggle_editing();
                    }
                }
                KeyCode::Char('e') => {
                    if self.context.prev_event_buf == Some(KeyCode::Tab)
                        && self.context.selected_subwindow.is_destination()
                    {
                        self.toggle_editing();
                    } else {
                        self.enter_char('e');
                        self.context.prev_event_buf = Some(KeyCode::Char('e'));
                    }
                }
                KeyCode::Left => {
                    self.move_cursor_left();
                    self.context.prev_event_buf = Some(KeyCode::Left);
                }
                KeyCode::Right => {
                    self.move_cursor_right();
                    self.context.prev_event_buf = Some(KeyCode::Right);
                }
                KeyCode::Backspace => {
                    self.delete_char();
                    self.context.prev_event_buf = Some(KeyCode::Backspace);
                }
                KeyCode::Char(to_insert) => self.enter_char(to_insert),

                _ => {}
            },
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

    pub fn toggle_editing(&mut self) {
        match self.context.input_mode {
            InputMode::Normal => self.context.input_mode = InputMode::Editing,
            InputMode::Editing => self.context.input_mode = InputMode::Normal,
        }
    }

    pub fn change_to_list(&mut self) {
        if self.status == Screen::Screen0 {
            self.status = Screen::Screen2;
        } else if self.status == Screen::Screen1 {
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
        new_cursor_pos.clamp(0, self.context.destination_temporary.chars().count())
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.context.destination_temporary.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn set_destination(&mut self, destination_temporary: String) {
        self.context.destination = destination_temporary;
    }

    pub fn byte_index(&self) -> usize {
        self.context
            .destination_temporary
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.context.selected_index)
            .unwrap_or(self.context.destination_temporary.len())
    }

    pub fn reset_cursor(&mut self) {
        self.context.selected_index = 0;
    }

    pub fn clear_message(&mut self) {
        self.context.destination_temporary.clear();
        self.reset_cursor();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.context.selected_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.context.selected_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self
                .context
                .destination_temporary
                .chars()
                .take(from_left_to_current_index);
            let after_char_to_delete = self.context.destination_temporary.chars().skip(current_index);

            self.context.destination_temporary = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn select_help_text(&mut self) {
        let text = match self.context.selected_subwindow {
            crate::ui::app_data::context::SubWindow::Destination => {
                vec![
                    Line::from("Keybinds:".fg(Color::Indexed(79)).bold()),
                    Line::from(" Press <Q> to quit application".fg(Color::Indexed(160))),
                    if self.context.input_mode.is_editing() {
                        Line::from(" Press <TAB + E> to disable editing".fg(Color::Indexed(175)))
                    } else {
                        Line::from(" Press <TAB + E> to enable editing".fg(Color::Indexed(105)))
                    },
                    Line::from(" Press <Enter> to send input".fg(Color::Indexed(117))),
                    Line::from(
                        " Press <TAB + (1-3)> to switch active subwindow".fg(Color::Indexed(135)),
                    ),
                    if self.context.input_mode.is_editing() {
                        Line::from(" Press <← or →> to move left and right".fg(Color::Indexed(35)))
                    }
                    else {
                        Line::from("".fg(Color::Indexed(35)))
                    },
                    if self.context.input_mode.is_editing() {
                        Line::from("\n <!> Max. 24 Input character ".fg(Color::Indexed(208)))
                    }
                    else {
                        Line::from(" <!> Max. 24 Input character ".fg(Color::Indexed(208)))
                    },
                ]
            }
            crate::ui::app_data::context::SubWindow::List => {
                vec![
                    Line::from("Keybinds:"),
                    Line::from(" Press <Q> to quit application"),
                ]
            }
            crate::ui::app_data::context::SubWindow::PickCard => {
                vec![
                    Line::from("Keybinds:"),
                    Line::from(" Press <Q> to quit application"),
                ]
            }
        };
        self.context.help_text = text;
    }
}
