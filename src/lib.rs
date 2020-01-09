pub mod client;
#[derive(Default)]
pub struct Color {pub r:  u8, pub g: u8, pub b:  u8, pub a: Option<u8>}
pub type Image = String;
impl ToString for Color {
    fn to_string(&self) -> String {
        let Color{r, g, b, a} = self;
        match a {
            Some(a) => format!("{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
            None => format!("{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

impl From<& image::Rgba<u8>> for Color {
    fn from(pixel: &image::Rgba<u8>) -> Self {
        let image::Rgba([r, g, b, a]) = *pixel;
        if a == 255 {
            Color{r, g, b, a: None}
        } else {
            Color{r, g, b, a: Some(a)}
        }
    }
}

#[derive(Default)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        format!("PX {} {} {} ", self.x, self.y, self.color.to_string())
    }
}
