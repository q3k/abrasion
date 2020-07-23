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
