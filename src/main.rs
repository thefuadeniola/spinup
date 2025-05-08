use clap::Parser;

pub mod new;
use new::new_project;

pub mod add;
use add::add_program;

use solana_spinup::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command{
        Commands::New { project_name } => {
            println!("Scaffolding new project in current directory, {}", project_name);
            new_project(&project_name);
        },
        Commands::Add { program_name } => {
            println!("Adding rust program: {}", program_name);
            add_program(&program_name);
        }
    }


}

// solana spinup lego
// /lego/program | /lego/client