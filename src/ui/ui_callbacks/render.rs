use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
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
}
