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

use std::sync::Arc;
use std::time;
use std::sync::Mutex;

#[derive(Default, Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tex: [f32; 2],
}

impl Vertex {
    pub fn new(pos: [f32; 3], normal: [f32; 3], tex: [f32; 2]) -> Self {
        Self {
            pos, normal, tex,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Arc<Vec<Vertex>>,
    pub indices: Arc<Vec<u16>>,
}

impl Mesh {
    pub fn new(
        vertices: Arc<Vec<Vertex>>,
        indices: Arc<Vec<u16>>,
    ) -> Self {
        Self {
            vertices, indices,
        }
    }
}
