use ratatui::{
    layout::{Margin, Rect},
    text::Line,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::ui::application::App;

impl App {
    pub fn application_render(
        &self,
        frame: &mut Frame,
        application_block: Block,
        application_area: Rect,
    ) {
        frame.render_widget(
            Paragraph::new("").block(application_block),
            application_area,
        );
    }

    pub fn destination_render(
        &self,
        frame: &mut Frame,
        destination_block: Block,
        destination_area: Rect,
    ) {
        frame.render_widget(
            Paragraph::new("").block(destination_block),
            destination_area,
        );
    }

    pub fn list_render(&self, frame: &mut Frame, list_block: Block, list_area: Rect) {
        frame.render_widget(Paragraph::new("").block(list_block), list_area);
    }

    pub fn pick_card_render(
        &self,
        frame: &mut Frame,
        pick_card_block: Block,
        pick_card_area: Rect,
    ) {
        frame.render_widget(Paragraph::new("").block(pick_card_block), pick_card_area);
    }

    pub fn help_render(&self, frame: &mut Frame, help_block: Block, help_area: Rect) {
        frame.render_widget(
            Paragraph::new(self.context.help_text.clone()).block(help_block),
            help_area,
        );
    }

    pub fn input_render(&self, frame: &mut Frame, input_block: Block, input_area: Rect) {
        frame.render_widget(
            Paragraph::new(self.context.destination_temporary.as_str()).block(input_block),
            input_area,
        );
    }

    pub fn list_inner_render(&mut self, frame: &mut Frame, list_inner_area: Rect) {
        frame.render_stateful_widget(
            self.list_inner_list(),
            list_inner_area,
            &mut self.context.list_state,
        );
    }

    pub fn pick_card_inner_render(
        &mut self,
        frame: &mut Frame,
        pick_card_inner_block: Block,
        pick_card_inner_area: Rect,
    ) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        self.get_scroll_state();

        let mut text = vec![];

        for (index, (set, name)) in self.context.content.iter().enumerate() {
            let index_display = match (index + 1).to_string().len() {
                1 => format!("{}  .", index + 1),
                2 => format!("{} .", index + 1),
                3 => format!("{}.", index + 1),
                _ => format!("{}.", index + 1),
            };

            text.insert(
                index,
                Line::from(format!(
                    "{index_display} <{}>: {name}",
                    set.to_ascii_uppercase()
                )),
            );
        }

        frame.render_widget(
            Paragraph::new(text.clone())
                .scroll((self.context.scroll_state as u16, 0))
                .block(pick_card_inner_block),
            pick_card_inner_area,
        );

        frame.render_stateful_widget(
            scrollbar,
            pick_card_inner_area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.context.scroll_struct_state,
        );
    }

    pub fn get_scroll_state(&mut self) {
        self.context.scroll_struct_state =
            ScrollbarState::new(self.context.content.len()).position(self.context.scroll_state);
    }
}
