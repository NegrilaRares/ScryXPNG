use application::app::App;

mod app_data;
mod ui_callbacks;
mod application;

pub fn run() {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.run(&mut terminal);
    ratatui::restore();
}


        //             let input_box = Block::bordered()
        //             .title(
        //                 Title::from(" Input Destination ./png_archive/... ".fg(ratatui::style::Color::Indexed(80)))
        //                     .alignment(Alignment::Center),
        //             )
        //             .border_set(border::ROUNDED)
        //             .style(
        //                 match self.context.input_mode { 
        //                     InputMode::Normal => Style::default(),
        //                     InputMode::Editing => Style::new().fg(ratatui::style::Color::Indexed(214)),
        //                 }
        //             ); 

        //         Paragraph::new(self.context.destination.as_str())
        //             .block(input_box)
        //             .render(display_border, buf);

        //         //General Display Border and Title
        //         // Paragraph::new("")
        //         //     .block(border_box)
        //         //     .render(display_border, buf);
        //     },