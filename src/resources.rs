use ggez::Context;

use crate::{fonts::Fonts, navigation_instructions::NavigationInstructions};

pub struct Resources {
    fonts: Fonts,
    navigation_instructions: NavigationInstructions,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> Self {
        let fonts = Fonts::load(ctx);
        Self {
            fonts,
            navigation_instructions: NavigationInstructions::new(fonts.light_italic),
        }
    }

    pub const fn get_fonts(&self) -> Fonts {
        self.fonts
    }

    pub const fn get_navigation_instructions(&self) -> &NavigationInstructions {
        &self.navigation_instructions
    }
}
