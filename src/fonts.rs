use ggez::{graphics::Font, Context};

use crate::app::log_error;

#[derive(Clone, Copy)]
pub struct Fonts {
    extra_bold: Font,
    semi_bold: Font,
    light_italic: Font,
}

impl Fonts {
    pub fn load(ctx: &mut Context) -> Self {
        Self {
            extra_bold: Self::load_font(ctx, "/ArgentumSans-ExtraBold.otf"),
            semi_bold: Self::load_font(ctx, "/ArgentumSans-SemiBold.otf"),
            light_italic: Self::load_font(ctx, "/ArgentumSans-LightItalic.otf"),
        }
    }

    fn load_font(ctx: &mut Context, filename: &str) -> Font {
        match Font::new(ctx, filename) {
            Ok(font) => font,
            Err(error) => {
                log_error("load_font", &error);
                panic!("{}", &error);
            }
        }
    }

    pub const fn get_extra_bold(&self) -> Font {
        self.extra_bold
    }

    pub const fn get_semi_bold(&self) -> Font {
        self.semi_bold
    }

    pub const fn get_light_italic(&self) -> Font {
        self.light_italic
    }
}
