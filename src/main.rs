use std::fs::File;
use std::io::stdout;
use std::io::Write;
use std::process::exit;
use structopt::clap::{crate_name, crate_version, Shell};
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
    /// Generate completion script
    Completion {
        /// Shell type
        #[structopt(possible_values = &["bash", "fish", "zsh", "powershell", "elvish"])]
        shell: Shell,
    },
}

enum Error {
    GopherNotFound(String),
    Response(String),
    IO(String),
}

impl From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Self {
        Error::Response(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err.to_string())
    }
}

fn get_gopher(gopher: String) -> Result<String, Error> {
    println!("Try to get {} Gopher...", gopher);
    let url = format!("https://github.com/scraly/gophers/raw/main/{}.png", gopher);
    let response = minreq::get(url).send()?;

    if response.status_code == 200 {
        let file_name = format!("{}.png", gopher);
        let mut output_file = File::create(&file_name)?;
        output_file.write_all(response.as_bytes())?;
        Ok(format!("Perfect! Just saved in {}", &file_name))
    } else {
        Err(Error::GopherNotFound(format!(
            "Gopher {} not exists",
            gopher
        )))
    }
}

fn display_error_and_exit(error_msg: String) {
    eprintln!("{}", error_msg);
    exit(255)
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
