use std::fs;
use std::process::Command;
use std::path::Path;


pub fn new_project(name: &String) {
    let project_dir = Path::new(name);

    // directory structure
    let program_path = project_dir.join("program");
    let src = program_path.join("src");
    let client = project_dir.join("client");

    fs::create_dir_all(&program_path).unwrap();
    fs::create_dir_all(&client).unwrap();
    fs::create_dir_all(&src).unwrap();
    

    // Cargo.toml
    let cargo_toml_content  = r#"[package]
name = "program"
version = "0.1.0"
edition = "2021"

[dependencies]
solana-program="1.9.0"
"#;


    let main_ts_content = "function main() {
    };
    
    main().then(
        () => process.exit(),
        err => {
            process.exit(-1)
        }
    )";

    let package_json = r#"
    {
      "name": "client",
      "version": "1.0.0",
      "main": "main.ts",
      "scripts": {
        "build": "tsc",
        "start": "node main.ts"
      },
      "dependencies": {
        "@solana/web3.js": "^1.98.2"
      },
      "devDependencies": {
        "typescript": "^5.0.0"
      }
    }
    "#;

    fs::write(program_path.join("Cargo.toml"), cargo_toml_content).unwrap();
    fs::write(src.join("lib.rs"), "").unwrap();
    fs::write(client.join("main.ts"), main_ts_content).unwrap();
    fs::write(project_dir.join("package.json"), package_json).unwrap();

    Command::new("npm")
            .arg("install")
            .current_dir(project_dir)
            .status()
            .expect("Failed to run npm install");

    Command::new("cargo")
            .arg("build")
            .current_dir(program_path)
            .status()
            .expect("Failed to run cargo build");


    println!("Project {} created! Run 'code .' to open it in your IDE", name);

}