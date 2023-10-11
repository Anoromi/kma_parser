use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command()]
pub struct Cli {

   #[command(subcommand)] 
    pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Parse {

        #[arg(short, long)]
        config: String,

        #[arg(short, long)]
        file: String,

        #[arg(short, long)]
        output: Option<String>

    }
}
