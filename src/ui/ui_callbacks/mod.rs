use ratatui::Frame;

use super::{app_data::Screen, application::App};

pub mod area;
pub mod block;
pub mod render;
pub mod style;

impl App {
    pub fn draw(&mut self, frame: &mut Frame) {
        let application_block = self.application_block();

        let destination_block = self.destination_block();
        let list_block = self.list_block();
        let pick_card_block = self.pick_card_block();
        let help_block = self.help_block();

        let input_block = self.input_block();
        let pick_card_inner_block = self.pick_card_inner_block();

        let application_area = area::application_area(frame.area());

        self.application_render(frame, application_block, application_area);

        match self.status {
            Screen::Screen0 => {
                let destination_area = area::partition_application_area_0(application_area);

                let input_area = area::input_area(destination_area);

                self.destination_render(frame, destination_block, destination_area);

                self.input_render(frame, input_block, input_area);
            }
            Screen::Screen1 => {
                let [destination_area, help_area] =
                    area::partition_application_area_1(application_area);

                let input_area = area::input_area(destination_area);

                self.destination_render(frame, destination_block, destination_area);
                self.help_render(frame, help_block, help_area);

                self.input_render(frame, input_block, input_area);
            }
            Screen::Screen2 => {
                let [destination_area, list_area] =
                    area::partition_application_area_2(application_area);

                let input_area = area::input_area(destination_area);
                let list_inner_area = area::list_inner_area(list_area);

                self.destination_render(frame, destination_block, destination_area);
                self.list_render(frame, list_block, list_area);

                self.input_render(frame, input_block, input_area);
                self.list_inner_render(frame, list_inner_area);
            }
            Screen::Screen3 => {
                let [destination_area, list_area, help_area] =
                    area::partition_application_area_3(application_area);

                let input_area = area::input_area(destination_area);
                let list_inner_area = area::list_inner_area(list_area);

                self.destination_render(frame, destination_block, destination_area);
                self.list_render(frame, list_block, list_area);
                self.help_render(frame, help_block, help_area);

                self.input_render(frame, input_block, input_area);
                self.list_inner_render(frame, list_inner_area);
            }
            Screen::Screen4 => {
                let [destination_area, list_area, pick_card_area] =
                    area::partition_application_area_4(application_area);

                let input_area = area::input_area(destination_area);
                let list_inner_area = area::list_inner_area(list_area);
                let pick_card_inner_area = area::pick_card_inner_area(pick_card_area);

                self.destination_render(frame, destination_block, destination_area);
                self.list_render(frame, list_block, list_area);
                self.pick_card_render(frame, pick_card_block, pick_card_area);

                self.input_render(frame, input_block, input_area);
                self.list_inner_render(frame, list_inner_area);
                self.pick_card_inner_render(frame, pick_card_inner_block, pick_card_inner_area);
            }
            Screen::Screen5 => {
                let [destination_area, list_area, pick_card_area, help_area] =
                    area::partition_application_area_5(application_area);

                let input_area = area::input_area(destination_area);
                let list_inner_area = area::list_inner_area(list_area);
                let pick_card_inner_area = area::pick_card_inner_area(pick_card_area);

                self.destination_render(frame, destination_block, destination_area);
                self.list_render(frame, list_block, list_area);
                self.pick_card_render(frame, pick_card_block, pick_card_area);
                self.help_render(frame, help_block, help_area);

                self.input_render(frame, input_block, input_area);
                self.list_inner_render(frame, list_inner_area);
                self.pick_card_inner_render(frame, pick_card_inner_block, pick_card_inner_area);
            }
        }
    }
}
