// Copyright 2020 Sergiusz 'q3k' Bazanski <q3k@q3k.org>
//
// This file is part of Abrasion.
//
// Abrasion is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, version 3.
//
// Abrasion is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// Abrasion.  If not, see <https://www.gnu.org/licenses/>.

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
    pub fn from_srgb(r: f32, g: f32, b: f32) -> Self {
        let (x, y, z) = srgb_to_cie_xyz(r, g, b);
        Self {
            x, y, z
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

/// Convert sRGB color space (in R8G8B8A8) to XYZ colorspace (floats).
pub fn srgb_packed_to_cie_xyz(rgba: u32) -> (f32, f32, f32, f32) {
    // Unpack RGBA to R, G, B, A 0-255 values.
    let r = ((rgba >> 24) & 0xff) as f32;
    let g = ((rgba >> 16) & 0xff) as f32;
    let b = ((rgba >> 8 ) & 0xff) as f32;
    let a = ((rgba      ) & 0xff) as f32;

    let r = r/255.0;
    let g = g/255.0;
    let b = b/255.0;

    let (x, y, z) = srgb_to_cie_xyz(r, g, b);
    (x, y, z, a)
}

/// Convert sRGB color space (floats) to XYZ colorspace (floats).
/// The input are floats in the range 0-1.0.
/// The output is a tuple of (X, Y, Z) floats.
pub fn srgb_to_cie_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    assert!(r >= 0.0 && r <= 1.0);
    assert!(g >= 0.0 && g <= 1.0);
    assert!(b >= 0.0 && b <= 1.0);

    // Following DIN EN 61966-2-1:2003-09
    // (which corresponds to IEC 61966-2-1:1999)
    // Equations (5) and (6)
    let eq56 = |v| {
        if v < 0.004045 {
            v / 12.92
        } else {
            let v: f32 = ((v + 0.055) / (1.055));
            v.powf(2.4) as f32
        }
    };
    let r = eq56(r);
    let g = eq56(g);
    let b = eq56(b);

    // Equation (7)
    // (unrolled matrix multiplication)
    let x = 0.4124 * r + 0.3576 * g + 0.1805 * b;
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let z = 0.0193 * r + 0.1192 * g + 0.9505 * b;

    (x, y , z)
}
