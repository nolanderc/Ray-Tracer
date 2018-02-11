
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}


impl Color {
    pub fn new(
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    ) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    pub fn zero() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }


    pub fn as_bytes(self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}

