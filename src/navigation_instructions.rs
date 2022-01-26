use ggez::graphics::{Color, Font, PxScale, Text, TextFragment};

use crate::constants::{COLOR_GRAY, NAVIGATION_INSTRUCTIONS_CHAR_SCALE};

/*******************************************************************************
**** NAVIGATION INSTRUCTIONS
*******************************************************************************/
#[derive(Clone)]
pub struct NavigationInstructions {
    playing_ready: Text,
    playing_go: Text,
    playing_pause: Text,
    playing_gameover: Text,

    go_back: Text,
}

impl NavigationInstructions {
    pub fn new(font: Font) -> Self {
        let label_factory = Factory::new(font);
        Self {
            // PLAYING
            playing_ready: label_factory.create_label("[Enter] to start\n[Esc] to main menu"),
            playing_go: label_factory.create_label("Press [Esc] to pause"),
            playing_pause: label_factory
                .create_label("[Enter] to continue\n[Esc] to exit to main menu"),
            playing_gameover: label_factory
                .create_label("[Enter] for a new game\n[Esc] to main menu"),
            // HOW TO PLAY & ABOUT
            go_back: label_factory.create_label("[Esc] to main menu"),
        }
    }

    pub const fn get_playing_ready(&self) -> &Text {
        &self.playing_ready
    }

    pub const fn get_playing_go(&self) -> &Text {
        &self.playing_go
    }

    pub const fn get_playing_pause(&self) -> &Text {
        &self.playing_pause
    }

    pub const fn get_playing_gameover(&self) -> &Text {
        &self.playing_gameover
    }

    pub const fn get_go_back(&self) -> &Text {
        &self.go_back
    }
}

/*******************************************************************************
**** NAVIGATION INSTRUCTIONS FACTORY
*******************************************************************************/
struct Factory {
    color: Option<Color>,
    font: Option<Font>,
    scale: Option<PxScale>,
}

impl Factory {
    pub fn new(font: Font) -> Self {
        Self {
            color: Some(COLOR_GRAY),
            font: Some(font),
            scale: Some(PxScale::from(NAVIGATION_INSTRUCTIONS_CHAR_SCALE)),
        }
    }

    fn create_label(&self, text: &str) -> Text {
        Text::new(TextFragment {
            text: text.to_string(),
            color: self.color,
            font: self.font,
            scale: self.scale,
        })
    }
}
