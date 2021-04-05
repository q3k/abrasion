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

use std::io::Read;

use runfiles::Runfiles;

#[derive(Debug)]
pub enum ResourceError {
    InvalidPath,
    NotFound,
    NoRunfiles,
    Other(std::io::Error),
}

type Result<T> = std::result::Result<T, ResourceError>;

pub enum Resource {
    File(std::io::BufReader<std::fs::File>),
}

impl Resource {
    pub fn string(&mut self) -> std::io::Result<String> {
        let mut contents = String::new();
        self.read_to_string(&mut contents);
        Ok(contents)
    }
}

impl std::io::Read for Resource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Resource::File(r) => r.read(buf)
        }
    }
}

impl std::io::BufRead for Resource {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            Resource::File(r) => r.fill_buf()
        }
    }
    fn consume(&mut self, amt: usize) {
        match self {
            Resource::File(r) => r.consume(amt)
        }
    }
}

impl std::io::Seek for Resource {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match self {
            Resource::File(r) => r.seek(pos)
        }
    }
}

pub fn resource<T>(name: T) -> Result<Resource>
where
    T: Into<String>
{
    let name: String = name.into();
    // Ensure name has //-prefix.
    let rel = name.strip_prefix("//").ok_or(ResourceError::InvalidPath)?;
    // Ensure no / prefix or suffix.
    if rel.starts_with("/") || rel.ends_with("/") {
        return Err(ResourceError::InvalidPath);
    }
    // Ensure no double slash elsewhere in the path.
    if rel.contains("//") {
        return Err(ResourceError::InvalidPath);
    }
    
    if let Ok(r) = Runfiles::create() {
        // TODO(q3k): unhardcode workspace name?
        let workspace = format!("abrasion/{}",  rel);
        let loc = r.rlocation(workspace);
        std::fs::File::open(loc).map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::NotFound => ResourceError::NotFound,
                _ => ResourceError::Other(e),
            }
        }).map(|f| {
            Resource::File(std::io::BufReader::new(f))
        })
    } else {
        return Err(ResourceError::NoRunfiles);
    }
}
