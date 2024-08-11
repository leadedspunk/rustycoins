use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() {
    println!("cargo:rustc-link-search=.\\libs");
    println!("cargo:rustc-link-lib=static=sqlite3");

    let out_dir = env::var("OUT_DIR").unwrap();
    // let out_path = Path::new(&out_dir);
    println!("OUT_DIR: {}", out_dir);

    let db_path = Path::new(&out_dir).join("./db.sqlite");

    // Check if the database already exists
    if !db_path.exists() {
        // Create the database file
        fs::File::create(&db_path).unwrap();

        // Run Diesel setup to initialize the database schema
        Command::new("diesel")
            .args(&["migration", "revert"])
            .env("DATABASE_URL", db_path.to_str().unwrap())
            .status()
            .expect("Failed to run diesel migration");

        Command::new("diesel")
            .args(&["migration", "run"])
            .env("DATABASE_URL", db_path.to_str().unwrap())
            .status()
            .expect("Failed to run diesel migration");
    }

    let en = Path::new("./.env");

    fs::copy(en, Path::new(&out_dir).join(".env"))
        .expect("Failed to copy env file");

    // Inform Cargo to rerun the build script if `db.sqlite` doesn't exist
    println!("cargo:rerun-if-changed={}", db_path.display());
}
