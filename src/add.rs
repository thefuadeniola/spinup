use std::fs;
use std::process::Command;
use std::path::Path;

pub fn add_program(name: &String) {
    let new_program = Path::new(name);
    let src = new_program.join("src");

    fs::create_dir_all(&new_program).unwrap();
    fs::create_dir_all(&src).unwrap();

    let cargo_toml_content  = format!(r#"[package]
    name = "{}"
    version = "0.1.0"
    edition = "2021"
    
    [dependencies]
    solana-program="1.9.0"
    "#, name);
    
    fs::write(new_program.join("Cargo.toml"), cargo_toml_content.as_str()).unwrap();
    fs::write(src.join("lib.rs"), "").unwrap();

    Command::new("cargo")
            .arg("build")
            .current_dir(new_program)
            .status()
            .expect("Failed to run cargo build");

    println!("Program {} added!", name)
}