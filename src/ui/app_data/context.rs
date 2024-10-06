use std::sync::{Arc, LazyLock, RwLock};

#[derive(Debug)]
pub struct Context {
    pub destination: String,
    pub input_mode: InputMode,
    pub selected_index: usize,
    pub help: bool,
    pub exit: bool,
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl InputMode {
    pub fn is_normal(&self) -> bool {
        matches!(self, InputMode::Normal)
    }

    pub fn is_editing(&self) -> bool {
        matches!(self, InputMode::Editing)
    }
}

impl Context {
    pub fn new() -> Context {
        Context {
            destination: String::new(),
            input_mode: InputMode::Normal,
            selected_index: 0,
            help: false,
            exit: false,
        }
    }
}