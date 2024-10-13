use ratatui::{
    style::{Color, Stylize},
    text::Line,
};

use super::App;

impl App {
    pub fn select_help_text(&mut self) {
        let text = match self.context.selected_subwindow {
            crate::ui::app_data::context::SubWindow::Destination => {
                vec![
                    Line::from("Keybinds:".fg(Color::Indexed(79)).bold()),
                    Line::from(" Press <Q> to quit application".fg(Color::Indexed(160))),
                    if self.context.input_mode.is_editing() {
                        Line::from(" Press <LSHIFT + E> to disable editing".fg(Color::Indexed(175)))
                    } else {
                        Line::from(" Press <LSHIFT + E> to enable editing".fg(Color::Indexed(105)))
                    },
                    Line::from(
                        " Press <LSHIFT + (1-3)> to switch active subwindow"
                            .fg(Color::Indexed(135)),
                    ),
                    if self.context.input_mode.is_editing() {
                        Line::from(" Press <← or →> to move left and right".fg(Color::Indexed(35)))
                    } else {
                        Line::from("".fg(Color::Indexed(35)))
                    },
                    if self.context.input_mode.is_editing() {
                        Line::from(" Press <Enter> to send input".fg(Color::Indexed(117)))
                    } else {
                        Line::from(String::new())
                    },
                    if self.context.input_mode.is_editing() {
                        Line::from("\n <!> Max. 19 Input character ".fg(Color::Indexed(208)))
                    } else {
                        Line::from(" <!> Max. 19 Input character ".fg(Color::Indexed(208)))
                    },
                ]
            }
            crate::ui::app_data::context::SubWindow::List => {
                vec![
                    Line::from("Keybinds:".fg(Color::Indexed(79)).bold()),
                    Line::from(" Press <Q> to quit application".fg(Color::Indexed(160))),
                    Line::from(
                        " Press <SHIFT + (1-3)> to switch active subwindow".fg(Color::Indexed(135)),
                    ),
                    Line::from(
                        " Press <↑ or ↓> to move up and down the list".fg(Color::Indexed(35)),
                    ),
                    Line::from(
                        " Press <Enter> to display card of the list".fg(Color::Indexed(117)),
                    ),
                ]
            }
            crate::ui::app_data::context::SubWindow::PickCard => {
                vec![
                    Line::from("Keybinds:"),
                    Line::from(" Press <Q> to quit application"),
                    Line::from(
                        " Press <SHIFT + (1-3)> to switch active subwindow".fg(Color::Indexed(135)),
                    ),
                    Line::from(
                        " Press <↑ or ↓> to move up and down the card list".fg(Color::Indexed(35)),
                    ),
                    Line::from(
                        " <!> Press <Enter> to start png fetching process".fg(Color::Indexed(208)),
                    ),
                    Line::from(" Press <Enter> to download card images".fg(Color::Indexed(117))),
                ]
            }
        };
        self.context.help_text = text;
    }
}
