mod gopher;

use gopher::*;
use simple_logger::SimpleLogger;
use std::io::stdout;
use std::process::exit;
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

fn display_error_and_exit(error_msg: String) {
    log::error!("{}", error_msg);
    exit(255)
}

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    let cmd = Command::from_args();
    log::debug!("{:#?}", cmd);
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => log::info!("{}", msg),
            Err(Error::GopherNotFound(msg)) => display_error_and_exit(msg),
            Err(Error::Response(msg)) => display_error_and_exit(msg),
            Err(Error::IO(msg)) => display_error_and_exit(msg),
        },
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}
