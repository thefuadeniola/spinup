use std::{fs, env};
use std::process::Command;
use std::path::Path;

pub fn add_program(name: &String) {
    let new_program = Path::new(name);
    let src = new_program.join("src");
    let current_dir = env::current_dir().unwrap();

    fs::create_dir_all(&new_program).unwrap();
    fs::create_dir_all(&src).unwrap();

    let cargo_toml_content  = format!(r#"[package]
    name = "{}"
    version = "0.1.0"
    edition = "2021"
    
    [dependencies]
    solana-program="1.9.0"
    "#, name);


    let script = format!(
        r#"
    #! /bin/bash
    
    SOLANA_PROGRAMS=("program", "{}")
    
    case $1 in
        "reset")
            rm -rf ./node_modules
            for x in $(solana program show --programs | awk 'RP==0 {{print $1}}'); do 
                if [[ $x != "Program" ]]; 
                then 
                    solana program close $x;
                fi
            done
            for program in "${{SOLANA_PROGRAMS[@]}}"; do
                cargo clean --manifest-path=./$program/Cargo.toml
            done
            rm -rf dist/program
            ;;
        "clean")
            rm -rf ./node_modules
            for program in "${{SOLANA_PROGRAMS[@]}}"; do
                cargo clean --manifest-path=./$program/Cargo.toml
            done;;
        "build")
            for program in "${{SOLANA_PROGRAMS[@]}}"; do
                cargo build-bpf --manifest-path=./$program/Cargo.toml --bpf-out-dir=./dist/program
            done;;
        "deploy")
            for program in "${{SOLANA_PROGRAMS[@]}}"; do
                cargo build-bpf --manifest-path=./$program/Cargo.toml --bpf-out-dir=./dist/program
                solana program deploy dist/program/$program.so
            done;;
        "reset-and-build")
            rm -rf ./node_modules
            for x in $(solana program show --programs | awk 'RP==0 {{print $1}}'); do 
                if [[ $x != "Program" ]]; 
                then 
                    solana program close $x; 
                fi
            done
            rm -rf dist/program
            for program in "${{SOLANA_PROGRAMS[@]}}"; do
                cargo clean --manifest-path=./$program/Cargo.toml
                cargo build-bpf --manifest-path=./$program/Cargo.toml --bpf-out-dir=./dist/program
                solana program deploy dist/program/$program.so
            done
            npm install
            solana program show --programs
            ;;
    esac
    "#,
        name
    );
        
    fs::write(new_program.join("Cargo.toml"), cargo_toml_content.as_str()).unwrap();
    fs::write(src.join("lib.rs"), "").unwrap();
    fs::write(current_dir.join("cicd.sh"), script).unwrap();

    Command::new("cargo")
            .arg("build")
            .current_dir(new_program)
            .status()
            .expect("Failed to run cargo build");

    println!("Program {} added!", name)
}