use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan {
        target: String,

        #[clap(short, long)]
        ports: String,
        #[clap(short = 't', long, default_value_t = 500)]
        timeout: u64,
        #[clap(short = 'c', long, default_value_t = 100)]
        concurrency: usize,
        #[arg(long, default_value_t = false)]
        cve: bool,
        #[clap(short, long)]
        output: Option<String>,

    }
}