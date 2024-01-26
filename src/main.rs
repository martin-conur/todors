use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: Option<String>,

        #[arg(short, long)]
        mandatory: bool
    }, 
    Delete {
        name: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();

   match cli.command {
    Commands::Add{name, mandatory} => {
        println!("myapp add was used, name is: {name:?}");
        if mandatory {println!("Mandatory: true")}
    },
    Commands::Delete { name } => {
        println!("Deleting task {name:?}")
    }
   }
}
