use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block,
    },
};

pub fn application_block() -> Block<'static> {
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

pub fn destination_block() -> Block<'static> {
    Block::bordered()
        .title(
            Title::from(" <1> Destination ".fg(ratatui::style::Color::Indexed(159)))
                .alignment(Alignment::Left),
        )
        .border_set(border::ROUNDED)
        .style(Style::new().fg(ratatui::style::Color::Indexed(33)))
}

pub fn list_block() -> Block<'static> {
    Block::bordered()
        .title(
            Title::from(" <2> Card List ".fg(ratatui::style::Color::Indexed(159)))
                .alignment(Alignment::Left),
        )
        .border_set(border::ROUNDED)
        .style(Style::new().fg(ratatui::style::Color::Indexed(33)))
}

pub fn pick_card_block() -> Block<'static> {
    Block::bordered()
        .title(
            Title::from(" <3> Pick Cards ".fg(ratatui::style::Color::Indexed(159)))
                .alignment(Alignment::Left),
        )
        .border_set(border::ROUNDED)
        .style(Style::new().fg(ratatui::style::Color::Indexed(33)))
}

pub fn help_block() -> Block<'static> {
    Block::bordered()
        .title(
            Title::from(" Help ".fg(ratatui::style::Color::Indexed(159)))
                .alignment(Alignment::Left),
        )
        .border_set(border::ROUNDED)
        .style(Style::new().fg(ratatui::style::Color::Indexed(33)))
}

pub fn input_block() -> Block<'static> {
    Block::bordered()
        .title(
            Title::from(
                " Input Destination: ./png_archive/... ".fg(ratatui::style::Color::Indexed(129)),
            )
            .alignment(Alignment::Center),
        )
        .border_set(border::ROUNDED)
        .style(Style::new().fg(ratatui::style::Color::Indexed(146)))
}
