use ratatui::style::Style;

pub fn focus_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(33))
}

pub fn unfocus_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(146))
}

pub fn editing_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(24))
}

pub fn nonediting_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(146))
}

pub fn help_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(162))
}
