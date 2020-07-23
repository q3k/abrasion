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

use std::path;

use runfiles::Runfiles;

pub fn resource_path(name: String) -> path::PathBuf {
    fn stringify(x: std::io::Error) -> String { format!("IO error: {}", x) }

    match Runfiles::create().map_err(stringify) {
        Err(_) => {
            let exe = std::env::current_exe().unwrap();
            let p = exe.parent().unwrap().join("..").join(name.clone());
            //let p = path::Path::new(".").join(name.clone());
            if !p.exists() {
                panic!("Could not load resource '{}', not found in runfiles or bare files (at {:?})", name, p);
            }
            log::info!("Loaded resource from bare file: {}", name);
            p
        },
        Ok(r) => {
            log::info!("Loaded resource from runfiles: {}", name.clone());
            r.rlocation(format!("abrasion/{}", name))
        }
    }
}
