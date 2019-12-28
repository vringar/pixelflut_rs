pub mod client;
#[derive(Default)]
pub struct Color(pub u8, pub u8, pub u8, pub Option<u8>);

impl ToString for Color {
    fn to_string(&self) -> String {
        let Color(r, g, b, a) = self;
        match a {
            Some(a) => format!("{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
            None => format!("{:02x}{:02x}{:02x}", r, g, b),
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
