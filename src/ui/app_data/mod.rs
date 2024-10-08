pub mod context;

#[derive(Debug, PartialEq)]
pub enum Screen {
    Screen0,
    Screen1,
    Screen2,
    Screen3,
    Screen4,
    Screen5,
}

impl Screen {
    pub fn is_0(&self) -> bool {
        matches!(self, Screen::Screen0)
    }

    pub fn is_1(&self) -> bool {
        matches!(self, Screen::Screen1)
    }

    pub fn is_2(&self) -> bool {
        matches!(self, Screen::Screen2)
    }

    pub fn is_3(&self) -> bool {
        matches!(self, Screen::Screen3)
    }

    pub fn is_4(&self) -> bool {
        matches!(self, Screen::Screen4)
    }

    pub fn is_5(&self) -> bool {
        matches!(self, Screen::Screen5)
    }
}
