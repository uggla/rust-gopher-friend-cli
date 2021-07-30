use structopt::clap::crate_version;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-gopher-friend-cli", version = crate_version!(), about = "Gopher CLI application written in Rust.")]
enum Command {
    /// This command will get the desired Gopher
    Get {
        /// Gopher type
        #[structopt()]
        gopher: String,
    },
}

fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => println!("{}", gopher),
    }
}
