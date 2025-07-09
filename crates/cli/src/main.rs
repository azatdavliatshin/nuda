use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nuda", version, about="The Nuda CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Scaffold a new Nuda project")]
    New {
        name: String,
    },
    Dev {},
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            log::info!("Creating new project: {}", name);
            if let Err(e) = scaffold::scaffold_project(name) {
                log::error!("❌ Failed to create project: {:#}", e);
                std::process::exit(1);
            }
        }
        Commands::Dev {} => {
            log::info!("Starting development server");
            if let Err(e) = dev::run_dev_server() {
                log::error!("❌ Failed to start development server: {:#}", e);
                std::process::exit(1);
            }
        }
    }
}
