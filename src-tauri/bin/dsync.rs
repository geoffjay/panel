use dsync::{GenerationConfig, TableOptions};
use std::{collections::HashMap, path::PathBuf};

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    dsync::generate_files(
        PathBuf::from_iter([dir, "src/schema.rs"]),
        PathBuf::from_iter([dir, "src/db/models"]),
        GenerationConfig {
            connection_type: "diesel::sqlite::SqliteConnection",
            options: Default::default(),
        },
    );
}
