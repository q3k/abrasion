use std::path;

use runfiles::Runfiles;

pub fn resource_path(name: String) -> path::PathBuf {
    fn stringify(x: std::io::Error) -> String { format!("IO error: {}", x) }

    match Runfiles::create().map_err(stringify) {
        Err(_) => path::Path::new(".").join(name),
        Ok(r) => r.rlocation(format!("abrasion/{}", name))
    }
}
