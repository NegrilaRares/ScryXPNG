use crossterm::event::KeyCode;
use ratatui::text::Line;

#[derive(Debug)]
pub struct Context {
    pub destination: String,
    pub destination_temporary: String,
    pub input_mode: InputMode,
    pub selected_index: usize,
    pub selected_subwindow: SubWindow,
    pub help_text: Vec<Line<'static>>,
    pub exit: bool,
    pub prev_event_buf: Option<KeyCode>,
}

#[derive(Debug)]
pub enum SubWindow {
    Destination,
    List,
    PickCard,
}

impl SubWindow {
    pub fn is_destination(&self) -> bool {
        matches!(self, SubWindow::Destination)
    }

    pub fn is_list(&self) -> bool {
        matches!(self, SubWindow::List)
    }

    pub fn is_pick_card(&self) -> bool {
        matches!(self, SubWindow::PickCard)
    }
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl InputMode {
    pub fn is_editing(&self) -> bool {
        matches!(self, InputMode::Editing)
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            destination: String::new(),
            destination_temporary: String::new(),
            input_mode: InputMode::Normal,
            selected_index: 0,
            selected_subwindow: SubWindow::Destination,
            exit: false,
            help_text: vec![],
            prev_event_buf: None,
        }
    }
}
