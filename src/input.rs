use ggez::event::KeyCode;

#[derive(Debug)]
pub enum InputEvent {
    None,
    Enter,
    Escape,
    LostFocus,
    Right,
    Left,
    Up,
    Down,
    Drop,
}

impl Default for InputEvent {
    fn default() -> Self {
        InputEvent::None
    }
}

impl InputEvent {
    pub fn map_input(keycode: KeyCode) -> Self {
        match keycode {
            // cursor keys
            KeyCode::Right => Self::Right,
            KeyCode::Left => Self::Left,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            // "wasd" keys
            KeyCode::D => Self::Right,
            KeyCode::A => Self::Left,
            KeyCode::W => Self::Up,
            KeyCode::S => Self::Down,
            // other keys
            KeyCode::Space => Self::Drop,
            KeyCode::Return => Self::Enter,
            KeyCode::Escape => Self::Escape,
            // default: None
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
