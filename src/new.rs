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
          "clean": "./cicd.sh clean",
          "reset": "./cicd.sh reset",
          "build": "./cicd.sh build",
          "deploy": "./cicd.sh deploy",
          "client": "ts-node ./client/main.ts"
      },
      "dependencies": {
        "@solana/web3.js": "^1.98.2",
        "borsh": "^0.7.0",
        "mz": "^2.7.0",
        "yaml": "^1.10.2"
      },
      "devDependencies": {
        "typescript": "^5.0.0"
      }
    }
    "#;


    let script: &'static str =
r#"
#! /bin/bash

SOLANA_PROGRAMS=("program")

case $1 in
    "reset")
        rm -rf ./node_modules
        for x in $(solana program show --programs | awk 'RP==0 {print $1}'); do 
            if [[ $x != "Program" ]]; 
            then 
                solana program close $x;
            fi
        done
        for program in "${SOLANA_PROGRAMS[@]}"; do
            cargo clean --manifest-path=./$program/Cargo.toml
        done
        rm -rf dist/program
        ;;
    "clean")
        rm -rf ./node_modules
        for program in "${SOLANA_PROGRAMS[@]}"; do
            cargo clean --manifest-path=./$program/Cargo.toml
        done;;
    "build")
        for program in "${SOLANA_PROGRAMS[@]}"; do
            cargo build-bpf --manifest-path=./$program/Cargo.toml --bpf-out-dir=./dist/program
        done;;
    "deploy")
        for program in "${SOLANA_PROGRAMS[@]}"; do
            cargo build-bpf --manifest-path=./$program/Cargo.toml --bpf-out-dir=./dist/program
            solana program deploy dist/program/$program.so
        done;;
    "reset-and-build")
        rm -rf ./node_modules
        for x in $(solana program show --programs | awk 'RP==0 {print $1}'); do 
            if [[ $x != "Program" ]]; 
            then 
                solana program close $x; 
            fi
        done
        rm -rf dist/program
        for program in "${SOLANA_PROGRAMS[@]}"; do
            cargo clean --manifest-path=./$program/Cargo.toml
            cargo build-bpf --manifest-path=./$program/Cargo.toml --bpf-out-dir=./dist/program
            solana program deploy dist/program/$program.so
        done
        npm install
        solana program show --programs
        ;;
esac
"#; 

  fs::write(program_path.join("Cargo.toml"), cargo_toml_content).unwrap();
  fs::write(src.join("lib.rs"), "").unwrap();
  fs::write(client.join("main.ts"), main_ts_content).unwrap();
  fs::write(project_dir.join("package.json"), package_json).unwrap();
  fs::write(project_dir.join("cicd.sh"), script).unwrap();

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