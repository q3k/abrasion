#[derive(Copy, Clone, Debug)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z,
        }
    }
}

pub struct sRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl sRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r, g, b,
        }
    }
}

pub struct LinearF32 {
    pub d: f32,
}

impl LinearF32 {
    pub fn new(d: f32) -> Self {
        Self {
            d,
        }
    }
}
