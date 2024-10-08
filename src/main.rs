use ui::application::App;

mod ui;

fn main() {
    env_logger::init();
    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.run(&mut terminal);
    ratatui::restore();
}

/*
    TODO
        - change tab to SHIFT DONE
        - continue list DONE
        - process list selected
        - list help text update
        - read data from txt file
        - create the pick card display

        - check for bugs 
*/
