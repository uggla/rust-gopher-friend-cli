use std::fs::File;
use std::io::stdout;
use std::io::Write;
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

fn get_gopher(gopher: String) {
    println!("Try to get {} Gopher...", gopher);
    let url = format!("https://github.com/scraly/gophers/raw/main/{}.png", gopher);
    let response = minreq::get(url)
        .send()
        .expect("Fail to get response from server");

    if response.status_code == 200 {
        let file_name = format!("{}.png", gopher);
        let mut output_file = File::create(&file_name).expect("Fail to create file");
        output_file
            .write_all(response.as_bytes())
            .expect("Fail to write file");
        println!("Perfect! Just saved in {}", &file_name);
    } else {
        eprintln!("Gopher {} not exists", gopher);
    }
}

fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => get_gopher(gopher),
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}
