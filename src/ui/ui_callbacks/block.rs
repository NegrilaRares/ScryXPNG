use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, List, ListDirection, Padding,
    },
};

use super::style;
use crate::ui::application::App;

impl App {
    pub fn application_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" ScryXPNG ".fg(ratatui::style::Color::Indexed(80)))
                    .alignment(Alignment::Center),
            )
            .title(
                Title::from(" Press <H> for help ".fg(ratatui::style::Color::Indexed(80)))
                    .position(Position::Bottom)
                    .alignment(Alignment::Center),
            )
            .border_set(border::ROUNDED)
            .style(Style::new().fg(ratatui::style::Color::Indexed(214)))
    }

    pub fn destination_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" <1> Destination ".fg(ratatui::style::Color::Indexed(159)))
                    .alignment(Alignment::Left),
            )
            .border_set(border::ROUNDED)
            .style(if self.context.selected_subwindow.is_destination() {
                style::focus_style()
            } else {
                style::unfocus_style()
            })
    }

    pub fn list_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" <2> Card List ".fg(ratatui::style::Color::Indexed(159)))
                    .alignment(Alignment::Left),
            )
            .border_set(border::ROUNDED)
            .style(if self.context.selected_subwindow.is_list() {
                style::focus_style()
            } else {
                style::unfocus_style()
            })
    }

    pub fn pick_card_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" <3> Pick Cards ".fg(ratatui::style::Color::Indexed(159)))
                    .alignment(Alignment::Left),
            )
            .border_set(border::ROUNDED)
            .style(if self.context.selected_subwindow.is_pick_card() {
                style::focus_style()
            } else {
                style::unfocus_style()
            })
    }

    pub fn help_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" Help ".fg(ratatui::style::Color::Indexed(159)))
                    .alignment(Alignment::Left),
            )
            .border_set(border::ROUNDED)
            .padding(Padding {
                left: 3,
                top: 1,
                right: 0,
                bottom: 0,
            })
            .style(style::help_style())
    }

    pub fn input_block(&mut self) -> Block<'static> {
        let title = format!(
            " Input Destination: ./png_archive/{} ",
            if self.context.destination_temporary.is_empty() && self.context.destination.is_empty()
            {
                "..."
            } else if !self.context.destination.is_empty()
                && self.context.destination_temporary.is_empty()
            {
                self.context.destination.as_str()
            } else {
                self.context.destination_temporary.as_str()
            }
        );

        Block::bordered()
            .title(
                Title::from(title.fg(ratatui::style::Color::Indexed(129)))
                    .alignment(Alignment::Center),
            )
            .border_set(border::ROUNDED)
            .style(if self.context.input_mode.is_editing() {
                style::editing_style()
            } else {
                style::nonediting_style()
            })
    }

    pub fn list_inner_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" Pick Card List: ".fg(ratatui::style::Color::Indexed(84)))
                    .alignment(Alignment::Left),
            )
            .border_set(border::ROUNDED)
            .style(if self.context.selected_subwindow.is_list() {
                style::editing_style()
            } else {
                style::nonediting_style()
            })
    }

    pub fn list_inner_list(&mut self) -> List<'static> {
        List::new(self.context.list_display.clone())
            .style(Style::default().fg(Color::Indexed(80)))
            .block(self.list_inner_block())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(" > ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
    }

    pub fn pick_card_inner_block(&mut self) -> Block<'static> {
        Block::bordered()
            .title(
                Title::from(" Cards - <SET>:<NAME> ".fg(ratatui::style::Color::Indexed(220)))
                    .alignment(Alignment::Left),
            )
            .border_set(border::ROUNDED)
            .style(if self.context.selected_subwindow.is_pick_card() {
                style::editing_style()
            } else {
                style::nonediting_style()
            })
    }
}
