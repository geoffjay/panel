use dsync::GenerationConfig;
use std::path::Path;

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    let result = dsync::generate_files(
        Path::new(&dir).join("src").join("schema.rs").as_path(),
        Path::new(&dir)
            .join("src")
            .join("db")
            .join("models")
            .as_path(),
        GenerationConfig {
            connection_type: "diesel::sqlite::SqliteConnection".to_string(),
            options: Default::default(),
        },
    );

    println!("{:?}", result);
}
