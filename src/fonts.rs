use ggez::{graphics::Font, Context};

use crate::app::log_error;

#[derive(Clone, Copy)]
pub struct Fonts {
    pub extra_bold: Font,
    pub semi_bold: Font,
    pub light_italic: Font,
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
}
