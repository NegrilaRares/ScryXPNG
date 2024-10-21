use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::ui::app_data::context::{InputMode, SubWindow};

use super::App;

impl App {
    pub fn handle_events(&mut self) {
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event.code, key_event.modifiers);
                }
                _ => {}
            }
        };
    }

    pub fn handle_key_event(&mut self, event: KeyCode, modifier: KeyModifiers) {
        // println!("Modifiers: {modifier:?} , KEY: {event:?}");
        match self.context.input_mode {
            InputMode::Normal => match event {
                KeyCode::Char('q') => {
                    self.exit();
                }
                KeyCode::Char('h') => {
                    self.toggle_help();
                }
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    if modifier == KeyModifiers::SHIFT
                        && self.context.selected_subwindow.is_destination()
                    {
                        self.toggle_editing();
                    }
                }
                KeyCode::Char('1') | KeyCode::Char('!') => {
                    if modifier == KeyModifiers::SHIFT {
                        self.context.selected_subwindow = SubWindow::Destination;
                    }
                }
                KeyCode::Char('2') | KeyCode::Char('@') => {
                    if modifier == KeyModifiers::SHIFT
                        && (self.status.is_2()
                            || self.status.is_3()
                            || self.status.is_4()
                            || self.status.is_5())
                    {
                        self.change_to_list();
                    }
                }
                KeyCode::Char('3') | KeyCode::Char('#') => {
                    if modifier == KeyModifiers::SHIFT && (self.status.is_4() || self.status.is_5())
                    {
                        self.change_to_cards();
                        self.cards_get_path();
                    }
                }
                KeyCode::Up => {
                    if self.context.selected_subwindow.is_list() {
                        if self.context.list_state.selected() == Some(0) {
                            self.list_select_last();
                        } else {
                            self.list_select_previous();
                        }
                    } else if self.context.selected_subwindow.is_pick_card() {
                        if self.context.scroll_state == 0 {
                            self.cards_select_last();
                        } else {
                            self.cards_select_previous();
                        }
                    }
                }
                KeyCode::Down => {
                    if self.context.selected_subwindow.is_list() {
                        if self.context.list_state.selected()
                            == Some(self.context.list_display.len() - 1)
                        {
                            self.list_select_first();
                        } else {
                            self.list_select_next();
                        }
                    } else if self.context.selected_subwindow.is_pick_card() {
                        if self.context.scroll_state == self.context.content.len() - 33 {
                            self.cards_select_first();
                        } else {
                            self.cards_select_next();
                        }
                    }
                }
                KeyCode::Enter => {
                    if self.context.selected_subwindow.is_list() && !self.context.lists.is_empty() {
                        self.change_to_cards();
                        self.cards_get_path();
                    } else if self.context.selected_subwindow.is_pick_card()
                        && !self.context.content.is_empty()
                    {
                        self.fetch_cards_url();
                        self.download_images();
                        self.change_to_destination();
                    }
                }
                _ => {}
            },

            InputMode::Editing => match event {
                KeyCode::Enter => {
                    if self.context.selected_subwindow.is_destination()
                        && !self.context.destination_temporary.is_empty()
                    {
                        if self.context.input_mode.is_editing() {
                            self.toggle_editing();
                        }
                        if self.status.is_0() || self.status.is_1() {
                            self.change_to_list();
                        }
                        self.set_destination(self.context.destination_temporary.clone());
                        self.clear_message();
                    }
                }
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    if modifier == KeyModifiers::SHIFT
                        && self.context.selected_subwindow.is_destination()
                    {
                        self.toggle_editing();
                    } else if self.context.selected_subwindow.is_destination()
                        && self.context.input_mode.is_editing()
                        && self.context.destination_temporary.len() < 19
                    {
                        self.enter_char('e');
                    }
                }
                KeyCode::Char('s') => {
                    if self.context.destination_temporary.len() < 19 {
                        self.enter_char('s');
                    }
                }
                KeyCode::Left => {
                    self.move_cursor_left();
                }
                KeyCode::Right => {
                    self.move_cursor_right();
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                KeyCode::Char(to_insert) => {
                    if self.context.destination_temporary.len() < 19 {
                        self.enter_char(to_insert)
                    }
                }
                _ => {}
            },
        }
    }
}
