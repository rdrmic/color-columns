use ggez::{graphics::Font, Context};

#[derive(Clone, Copy)]
pub struct Fonts {
    extra_bold: Font,
    semi_bold: Font,
    light_italic: Font,
}

impl Fonts {
    pub fn load(ctx: &mut Context) -> Self {
        Self {
            extra_bold: Font::new(ctx, "/ArgentumSans-ExtraBold.otf").unwrap(),
            semi_bold: Font::new(ctx, "/ArgentumSans-SemiBold.otf").unwrap(),
            light_italic: Font::new(ctx, "/ArgentumSans-LightItalic.otf").unwrap(),
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
