use clap::Parser;
use args::Args;

/// Contains types for parsing the args at startup. 
pub mod args;
/// Contains abstractions for interacting with the console. 
pub mod console;


fn main() {
    let cli = Args::parse();
    println!("{}", cli.show_state());
}
