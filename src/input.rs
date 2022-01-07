use ggez::event::KeyCode;

#[derive(Debug)]
pub enum Event {
    None,
    Enter,
    Escape,
    LostFocus,
    SaveScoreOnQuit,
    Right,
    Left,
    Up,
    Down,
    Drop,
}

impl Default for Event {
    fn default() -> Self {
        Self::None
    }
}

impl Event {
    pub fn map_input(keycode: KeyCode) -> Self {
        match keycode {
            KeyCode::Right | KeyCode::D => Self::Right,
            KeyCode::Left | KeyCode::A => Self::Left,
            KeyCode::Up | KeyCode::W => Self::Up,
            KeyCode::Down | KeyCode::S => Self::Down,
            KeyCode::Space => Self::Drop,
            KeyCode::Return => Self::Enter,
            KeyCode::Escape => Self::Escape,
            _ => Self::default(),
        }
    }

    /*pub fn __print(input_event: &InputEvent) {
        match input_event {
            InputEvent::None => (),
            _ => {
                println!("--> {:?}", input_event);
            }
        }
    }*/
}
