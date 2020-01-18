To do Rust vendoring first run:

    cargo install cargo-vendor
    cargo install cargo-raze

After changing Rust deps in Cargo.toml do:

    cargo generate-lockfile
    cargo-vendor vendor -x
    cargo raze
