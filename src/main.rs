mod gopher;

use gopher::*;

use std::io::stdout;
use structopt::clap::{crate_name, crate_version, Shell};
use structopt::StructOpt;

const BASE_URL: &str = "https://github.com/scraly/gophers/raw/main";

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-gopher-friend-cli", version = crate_version!(), about = "Gopher CLI application written in Rust.")]
enum Command {
    /// This command will get the desired Gopher
    Get {
        /// Gopher type
        #[structopt()]
        gopher: String,
    },
    /// Generate completion script
    Completion {
        /// Shell type
        #[structopt(possible_values = &["bash", "fish", "zsh", "powershell", "elvish"])]
        shell: Shell,
    },
}

fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => println!("{}", msg),
            Err(Error::GopherNotFound(msg)) => eprintln!("{}", msg),
            Err(Error::Response(msg)) => display_error_and_exit(msg),
            Err(Error::IO(msg)) => display_error_and_exit(msg),
        },
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}
