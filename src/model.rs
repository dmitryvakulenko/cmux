use crate::tea::Message;

pub struct Model {
    pub quit: bool
}

impl Model {
    pub fn new() -> Self {
        Self {
            quit: false,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Quit => self.quit = true,
            Message::None => (),
        }
    }
}