use ratatui::style::Style;

fn focus_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(214))
}

fn unfocus_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(214))
}

fn editing_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(214))
}

fn nonediting_style() -> Style {
    Style::new().fg(ratatui::style::Color::Indexed(214))
}