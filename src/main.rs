use ui::application::App;

mod ui;

fn main() {
    env_logger::init();

    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.run(&mut terminal);
    ratatui::restore();
}
