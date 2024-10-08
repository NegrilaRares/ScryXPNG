use std::fs;

use crate::ui::app_data::{
    context::{InputMode, SubWindow},
    Screen,
};

use super::App;

impl App {
    //
    // GENERAL CONTEXT MANIPULATION COMMANDS
    //
    pub fn change_to_list(&mut self) {
        if self.status == Screen::Screen0 {
            self.read_list_dir();
            self.select_list_display();
            self.status = Screen::Screen2;
            self.context.selected_subwindow = SubWindow::List;
        } else if self.status == Screen::Screen1 {
            self.read_list_dir();
            self.select_list_display();
            self.status = Screen::Screen3;
            self.context.selected_subwindow = SubWindow::List;
        } else if (self.status == Screen::Screen4
            || self.status == Screen::Screen5
            || self.status == Screen::Screen2
            || self.status == Screen::Screen3)
            && !self.context.selected_subwindow.is_list()
        {
            self.read_list_dir();
            self.select_list_display();
            self.context.selected_subwindow = SubWindow::List;
        }
    }

    pub fn change_to_cards(&mut self) {
        if self.status == Screen::Screen2 {
            self.status = Screen::Screen4;
            self.context.selected_subwindow = SubWindow::PickCard;
        } else if self.status == Screen::Screen3 {
            self.status = Screen::Screen5;
            self.context.selected_subwindow = SubWindow::PickCard;
        } else if (self.status == Screen::Screen4 || self.status == Screen::Screen5)
            && !self.context.selected_subwindow.is_pick_card()
        {
            self.context.selected_subwindow = SubWindow::PickCard;
        }
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

    //
    // INPUT COMMANDS
    //
    // INPUT FIELD
    pub fn exit(&mut self) {
        self.context.exit = true;
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
            let after_char_to_delete = self
                .context
                .destination_temporary
                .chars()
                .skip(current_index);

            self.context.destination_temporary =
                before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    // INPUT CONTEXT
    pub fn toggle_editing(&mut self) {
        match self.context.input_mode {
            InputMode::Normal => self.context.input_mode = InputMode::Editing,
            InputMode::Editing => self.context.input_mode = InputMode::Normal,
        }
    }

    //
    // LIST COMMANDS
    //
    // INPUT FIELD
    pub fn read_list_dir(&mut self) {
        let paths = fs::read_dir("./card_lists").unwrap();

        for (index, path) in paths.enumerate() {
            self.context.lists.insert(index, path.unwrap());
        }
    }

    pub fn select_list_display(&mut self) {
        for (index, path) in self.context.lists.iter().enumerate() {
            self.context.list_display.insert(
                index,
                path.path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into(),
            );
        }
    }

    pub fn list_select_next(&mut self) {
        self.context.list_state.select_next();
    }

    pub fn list_select_previous(&mut self) {
        self.context.list_state.select_previous();
    }

    pub fn list_select_first(&mut self) {
        self.context.list_state.select_first();
    }

    pub fn list_select_last(&mut self) {
        self.context.list_state.select_last();
    }

    //
    // CARDS COMMANDS
    //
    pub fn cards_get_path(&mut self) {
        let temp_selected = self.context.list_state.selected().unwrap();
        let temp_path = self.context.lists.get(temp_selected).unwrap().path();
        self.context.path = Some(temp_path);
        self.path_to_text();
    }

    pub fn path_to_text(&mut self) {
        let temp_string = fs::read_to_string(self.context.path.as_mut().unwrap()).unwrap();

        let mut temp_content: Vec<(String, String)> = vec![];

        for (index, line) in temp_string.lines().enumerate() {
            let mut text = line.split("%");
            let set: String = text.next().unwrap().to_string();
            let name: String = text.next().unwrap().to_string();
            temp_content.insert(index, (set, name));
        }

        self.context.content = temp_content;
    }

    pub fn cards_select_next(&mut self) {
        self.context.scroll_state += 1;
    }

    pub fn cards_select_previous(&mut self) {
        self.context.scroll_state -= 1;
    }

    pub fn cards_select_first(&mut self) {
        self.context.scroll_state = 0;
    }

    pub fn cards_select_last(&mut self) {
        self.context.scroll_state = self.context.content.len() - 33;
    }
}
