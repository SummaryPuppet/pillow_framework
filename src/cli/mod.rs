use clap::{Parser, Subcommand};

/// Cli for pillow framework
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init a project
    Init {
        #[arg(short, long)]
        /// Name of project
        name: String,
    },
    /// Make
    Make {
        #[arg(short, long)]
        /// Controller for routes
        controller: String,
    },
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Init { name }) => {
                println!("{name}")
            }
            Some(Commands::Make { controller }) => {
                println!("Controller {controller} created")
            }
            None => {}
        }
    }
}
