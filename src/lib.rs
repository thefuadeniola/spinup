use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="spinup", about="Scaffold a solana project", version="0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        #[arg(help = "Add the name of the project dir. spinup new <PROJECT_NAME>")]
        project_name: String
    },
    Add {
        #[arg(help = "Add the name of the extra rust program you need. spinup add <PROGRAM_NAME")]
        program_name: String
    }
}