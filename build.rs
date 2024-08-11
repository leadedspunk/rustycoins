use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() {
    println!("cargo:rustc-link-search=.\\libs");
    println!("cargo:rustc-link-lib=static=sqlite3");

    let out_dir = env::var("OUT_DIR").unwrap();
    // let out_path = Path::new(&out_dir);
    let out_dir2 = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    let db_path = out_dir2.join("db.sqlite");
    println!("{}", &db_path.to_str().unwrap());
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

    // Inform Cargo to rerun the build script if `db.sqlite` doesn't exist
    println!("cargo:rerun-if-changed={}", db_path.display());
}
