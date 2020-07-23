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
