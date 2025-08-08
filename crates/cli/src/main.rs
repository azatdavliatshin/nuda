use std::path::PathBuf;

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
    New { name: String },
    #[command(about = "Start a development server")]
    Dev {
        #[arg(long)]
        dir: Option<PathBuf>,
    },
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
        Commands::Dev { dir } => {
            // 1) Determine project root
            let root = dir
                .clone()
                .unwrap_or_else(|| std::env::current_dir().expect("Failed to get cwd"));
            if !shared::config::is_project_root(&root) {
                eprintln!(
                    "❌ The current directory is not a Nuda project: {}\n\
                     Hint: run this command from the project root or specify it via --dir <path>",
                    root.display()
                );
                std::process::exit(2);
            }

            // 2) Load config and validate
            let cfg =
                shared::config::load_config_from(&root).expect("Failed to load nuda.config.json");
            let app_name = cfg.name.as_ref().unwrap_or_else(|| {
                eprintln!("❌ Missing 'name' field in nuda.config.json");
                std::process::exit(3);
            });

            let host = cfg
                .dev
                .as_ref()
                .and_then(|d| d.host.as_deref())
                .unwrap_or("127.0.0.1");

            // 3) Change cwd to project root so relative paths work
            std::env::set_current_dir(&root).expect("Failed to change directory to project root");

            let port = dev::port::dev_port();
            println!(
                "🚀 {} is running at http://{}:{}/  (root: {})",
                app_name,
                host,
                port,
                std::env::current_dir().unwrap().display()
            );

            // 4) Run dev server
            if let Err(e) = dev::run_dev_server() {
                log::error!("❌ Failed to start development server: {:#}", e);
                std::process::exit(1);
            }
        }
    }
}
