use ratatui::{
    text::{Line, Text},
    widgets::{ListState, ScrollbarState},
};
use serde::Deserialize;
use std::{fs::DirEntry, path::PathBuf};

#[derive(Debug)]
pub struct Context {
    pub selected_subwindow: SubWindow,
    pub help_text: Vec<Line<'static>>,
    pub exit: bool,

    // Destination Input Necessary data
    pub input_mode: InputMode,
    pub destination: String,
    pub destination_temporary: String,
    pub selected_index: usize,

    // List Necessary data
    pub lists: Vec<DirEntry>,
    pub list_display: Vec<Text<'static>>,
    pub list_state: ListState,
    pub path: Option<PathBuf>,
    pub content: Vec<(String, String)>,

    // Cards Necessary data
    pub scroll_state: usize,
    pub scroll_struct_state: ScrollbarState,

    // Scryfall API
    pub card_url: Vec<Card>,
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

#[derive(serde::Deserialize, Debug)]
pub struct Card {
    pub image_uris: ImageUris,
}

#[derive(Deserialize, Debug)]
pub struct ImageUris {
    pub png: String,
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
        let mut list_state = ListState::default();
        list_state.select_first();

        Context {
            destination: String::new(),
            destination_temporary: String::new(),
            input_mode: InputMode::Normal,
            selected_index: 0,
            selected_subwindow: SubWindow::Destination,
            exit: false,
            help_text: vec![],
            lists: vec![],
            list_display: vec![],
            list_state,
            path: None,
            content: vec![],
            scroll_state: 0,
            scroll_struct_state: ScrollbarState::new(0),
            card_url: vec![],
        }
    }
}
