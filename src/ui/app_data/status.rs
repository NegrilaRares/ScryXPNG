use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Paragraph, Widget}};
use crate::ui::ui_callbacks::{area, block};

#[derive(Debug)]
pub enum States {
    Screen0(Screen0),
    Screen1(Screen1),
    Screen2(Screen2),
    Screen3(Screen3),
    Screen4(Screen4),
    Screen5(Screen5),
}





#[derive(Debug)]
pub struct Screen0 {
    pub application_block: Block<'static>,
    pub destination_block: Block<'static>,
    pub input_block: Block<'static>
}

#[derive(Debug)]
pub struct Screen1 {
    pub application_block: Block<'static>,
    pub destination_block: Block<'static>,
    pub help_block: Block<'static>,
    pub input_block: Block<'static>
}

#[derive(Debug)]
pub struct Screen2 {
    pub application_block: Block<'static>,
    pub destination_block: Block<'static>,
    pub list_block: Block<'static>,
    pub input_block: Block<'static>
}

#[derive(Debug)]
pub struct Screen3 {
    pub application_block: Block<'static>,
    pub destination_block: Block<'static>,
    pub list_block: Block<'static>,
    pub help_block: Block<'static>,
    pub input_block: Block<'static>
}

#[derive(Debug)]
pub struct Screen4 {
    pub application_block: Block<'static>,
    pub destination_block: Block<'static>,
    pub list_block: Block<'static>,
    pub pick_card_block: Block<'static>,
    pub input_block: Block<'static>
}

#[derive(Debug)]
pub struct Screen5 {
    pub application_block: Block<'static>,
    pub destination_block: Block<'static>,
    pub list_block: Block<'static>,
    pub pick_card_block: Block<'static>,
    pub help_block: Block<'static>,
    pub input_block: Block<'static>
}



impl Screen0 {
    pub fn new() -> Screen0 {
        Screen0 {
            application_block: block::application_block(),
            destination_block: block::destination_block(),
            input_block: block::input_block(),
        }
    }
}

impl Screen1 {
    pub fn new() -> Screen1 {
        Screen1 {
            application_block: block::application_block(),
            destination_block: block::destination_block(),
            help_block: block::help_block(),
            input_block: block::input_block(),
        }
    }
}

impl Screen2 {
    pub fn new() -> Screen2 {
        Screen2 {
            application_block: block::application_block(),
            destination_block: block::destination_block(),
            list_block: block::list_block(),
            input_block: block::input_block(),
        }
    }
}

impl Screen3 {
    pub fn new() -> Screen3 {
        Screen3 {
            application_block: block::application_block(),
            destination_block: block::destination_block(),
            list_block: block::list_block(),
            help_block: block::help_block(),
            input_block: block::input_block(),
        }
    }
}

impl Screen4 {
    pub fn new() -> Screen4 {
        Screen4 {
            application_block: block::application_block(),
            destination_block: block::destination_block(),
            list_block: block::list_block(),
            pick_card_block: block::pick_card_block(),
            input_block: block::input_block(),
        }
    }
}

impl Screen5 {
    pub fn new() -> Screen5 {
        Screen5 {
            application_block: block::application_block(),
            destination_block: block::destination_block(),
            list_block: block::list_block(),
            pick_card_block: block::pick_card_block(),
            help_block: block::help_block(),
            input_block: block::input_block(),
        }
    }
}




impl Widget for &Screen0 {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let application_area = area::application_area(area);
            let destination_area = area::partition_application_area_0(application_area);
                let input_area = area::input_area(destination_area);
        
        Paragraph::new("")
                    .block(self.application_block.clone())
                    .render(application_area, buf);

        Paragraph::new("")
            .block(self.destination_block.clone())
            .render(destination_area, buf);

        Paragraph::new("")
            .block(self.input_block.clone())
            .render(input_area, buf);
    }
}

impl Widget for &Screen1 {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let application_area = area::application_area(area);
            let [destination_area, help_area] = area::partition_application_area_1(application_area);
                let input_area = area::input_area(destination_area);
        
        Paragraph::new("")
            .block(self.application_block.clone())
            .render(application_area, buf);

        Paragraph::new("")
            .block(self.destination_block.clone())
            .render(destination_area, buf);

        Paragraph::new("")
            .block(self.help_block.clone())
            .render(help_area, buf);

        Paragraph::new("")
            .block(self.input_block.clone())
            .render(input_area, buf);
    }
}

impl Widget for &Screen2 {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let application_area = area::application_area(area);
            let [destination_area, list_area] = area::partition_application_area_2(application_area);
                let input_area = area::input_area(destination_area);
        
        Paragraph::new("")
            .block(self.application_block.clone())
            .render(application_area, buf);

        Paragraph::new("")
            .block(self.destination_block.clone())
            .render(destination_area, buf);

        Paragraph::new("")
            .block(self.list_block.clone())
            .render(list_area, buf);

        Paragraph::new("")
            .block(self.input_block.clone())
            .render(input_area, buf);
    }
}

impl Widget for &Screen3 {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let application_area = area::application_area(area);
            let [destination_area, list_area, help_area] = area::partition_application_area_3(application_area);
                let input_area = area::input_area(destination_area);

        Paragraph::new("")
            .block(self.application_block.clone())
            .render(application_area, buf);

        Paragraph::new("")
            .block(self.destination_block.clone())
            .render(destination_area, buf);

        Paragraph::new("")
            .block(self.list_block.clone())
            .render(list_area, buf);

        Paragraph::new("")
            .block(self.help_block.clone())
            .render(help_area, buf);

        Paragraph::new("")
            .block(self.input_block.clone())
            .render(input_area, buf);
    }
}

impl Widget for &Screen4 {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let application_area = area::application_area(area);
            let [destination_area, list_area, pick_card_area] = area::partition_application_area_4(application_area);
                let input_area = area::input_area(destination_area);

        Paragraph::new("")
            .block(self.application_block.clone())
            .render(application_area, buf);

        Paragraph::new("")
            .block(self.destination_block.clone())
            .render(destination_area, buf);

        Paragraph::new("")
            .block(self.list_block.clone())
            .render(list_area, buf);

        Paragraph::new("")
            .block(self.pick_card_block.clone())
            .render(pick_card_area, buf);

        Paragraph::new("")
            .block(self.input_block.clone())
            .render(input_area, buf);
    }
}

impl Widget for &Screen5 {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let application_area = area::application_area(area);
            let [destination_area, list_area, pick_card_area, help_area] = area::partition_application_area_5(application_area);
                let input_area = area::input_area(destination_area);

        Paragraph::new("")
            .block(self.application_block.clone())
            .render(application_area, buf);

        Paragraph::new("")
            .block(self.destination_block.clone())
            .render(destination_area, buf);

        Paragraph::new("")
            .block(self.list_block.clone())
            .render(list_area, buf);

        Paragraph::new("")
            .block(self.pick_card_block.clone())
            .render(pick_card_area, buf);

        Paragraph::new("")
            .block(self.help_block.clone())
            .render(help_area, buf);

        Paragraph::new("")
            .block(self.input_block.clone())
            .render(input_area, buf);
    }
}

