## Why this article
This article is inspired by Aurelie Vache's [learning-go-by-examples-part-3-create-a-cli-app-in-go](https://dev.to/aurelievache/learning-go-by-examples-part-3-create-a-cli-app-in-go-1h43).
Aurelie is a proficient devrel at Stacklab, she writes really good and didactic articles around technologies like Kubernetes, Istio, Go...
She also makes nice funny drawing to illustrate all these concepts and make an easier learning experience.
If you don't already follow Aurelie's content, I strongly encourage you to do it.

The article above is a cli example written in go. Looking at it, I wondered how it will look like written in Rust.

Rust has the reputation to be a language with a hard learning curve, but on such example is it really the case ?
Would the code be longer ? More complex ? Harder to read ?

So I wrote almost the same cli example regarding functionalities to have an idea and share the result, so you will be able to forge your opinion about it.

## Disclaimer
The goal of this article is not to compare or tell which language is the best. To my mind this is endless discussion and pure waste of time. Languages are tools and choosing the most efficient one is a matter of use cases and personal preferences.
Is a hammer better than a crowbar ? It does not make any sens, it really depends what you want to achieve.

Also, this article will not explain the famous Rust borrow checker and the Rust language. You will find tons of articles better than this one about it.

Finally this article assumes that you have a Rust environment available on your system. Look at https://www.rust-lang.org/learn/get-started that will give you the instruction to do it.

## What our cli will do ?
This is a cli example that will have 2 commands:
* The get command. It will require a value which is the name of a file. Then it will connect to one of Aurelie's github repository hosting "gopher postcards" png files. Finally it will download the file and store it locally, if the file is available.
* The completion command will allow to get a completion script that we will be able to source in order to have our program shell completion.

## Starting with the cli
Rust equivalent to go modules are called crates. You can search crates on the [crates.io](https://crates.io/) web site.

The usual crate for managing cli is called clap. This is an awesome tool that quickly allows to code a cli. (believe me, if a French guy says it is awesome, this is awesome ! :))
It covers a lot of cli functionalities from simple cases to complex interfaces with various commands and options.

But as I'm a lazy guy, there is another even more effortless option called structopt.
Structopt is an over layer of clap. With structopt, coding the cli is more or less writing a correct data structure that will be processed to generate the cli.

###  Cli and get command implementation

* Create the project.
```bash
cargo new --bin rust-gopher-friend-cli
cd rust-gopher-friend-cli
```

* Add the structop dependence to Cargo.toml
You can either edit the Cargo.toml adding structopt dependency in the dependencies section.
```toml
[dependencies]
structopt = "0.3.22"
```
or install the cargo-add plugin and use it
```bash
cargo install cargo-add
cargo add structopt --version 0.3.22
```
*Note: Look at https://crates.io/categories/development-tools::cargo-plugins?sort=alpha to get a list of Cargo plugins.*

You can use a "*" regarding the release version and the latest version will be used, however specifying the release needed allows to better control upgrades.

* So now the minimal code to manage our get command
```rust
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
```

#### Let's explain this block code step by step
* Import the crate_version macro.
* Import the StrucOpt trait.

```rust
use structopt::clap::crate_version;
use structopt::StructOpt;
```

* As explained before we need to define our cli with the correct data structure. In this example, we just want to have the get command. This can be achieved with an enum that will describe the command.
* The enum structure needs to be surrounded by attributes `#[derive(StructOpt, Debug)]` tells that the "enum" will be managed by structopt.
* The second attribute `#[structopt(name = "rust-gopher-friend...` will defined the parameter of our cli.
* The comments starting with a `///` will simply be used to describe our commands.

```rust
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
```

* Within the main, we use the from_args() method to create our structopt enum from the command line arguments.
* Then we use the pattern matching on the enum to extract the parameter value passed on the cli.

```
fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => println!("{}", gopher),
    }
}
```

#### Time to run this simple example

The following command will compile and run our short example and invoke it with the `--help` parameter.

```bash
cargo run -- --help
```
Output:
```bash
   Compiling rust-gopher-friend-cli v0.1.0 (/home/uggla/workspace/rust/rust-gopher-friend-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 1.11s
     Running `target/debug/rust-gopher-friend-cli --help`
rust-gopher-friend-cli 0.1.0
Gopher CLI application written in Rust.

USAGE:
    rust-gopher-friend-cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get     This command will get the desired Gopher
    help    Prints this message or the help of the given subcommand(s)
```

So we can see that the cli help was generated properly with the information passed into the enum definition.

We can see as well that error handling was generated too:
```bash
cargo run -- put
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust-gopher-friend-cli put`
error: Found argument 'put' which wasn't expected, or isn't valid in this context

USAGE:
    rust-gopher-friend-cli <SUBCOMMAND>

For more information try --help
```

And if we use our cli correctly:
```bash
cargo run -- get my-gopher-name
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-gopher-friend-cli get my-gopher-name`
my-gopher-name
```

It prints what we passed on the command line as value for the get command.


### Add the completion command

There are two options for generating completion script:
* At compile time --> this will produce a static completion script.
* At build time --> completion script will be generated running the program.

Here we will use the second option to better mimic the original go code.

```rust
use std::io::stdout;
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

fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => println!("{}", gopher),
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}

```
#### Let's explain step by step what we have added compared to the previous code

* We import `stdout` function, `crate_name` macro and `Shell` enum from structopt::clap.
* The stdout function will return a handle to the standard output. The handle is a reference to a shared global buffer.
```rust
use std::io::stdout;
use structopt::clap::{crate_name, crate_version, Shell};
```
* We add the completion command in the enum definition specifying that the expected value must be defined by the Shell enum (bash, zsh...).
* We define the possible values in an attribute `#[structopt(possible_values = &["bash", "fish", "zsh", "powershell", "elvish"])]` this is not really necessary regarding the error handling. A value entered not defined by the Shell enum will produce an error. *Maybe it is a bug or I miss something, but without this declaration the possible values are not printed in the help message.*
```rust
    /// Generate completion script
    Completion {
        /// Shell type
        #[structopt(possible_values = &["bash", "fish", "zsh", "powershell", "elvish"])]
        shell: Shell,
    },
```
* In the main, we call the gen_completions_to() method to generate the completion script for the required shell, if the completion command is invoked.
* The gen_completions_to() method takes 3 arguments: The name of the program, the shell type (bash, zsh...), a buffer to output the completion script.
* So we can use the stdout() function as the third parameter.
```rust
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
```

#### Check completion script is working

* Checking the help:
```bash
cargo run -- completion --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-gopher-friend-cli completion --help`
rust-gopher-friend-cli-completion 0.1.0
Generate completion script

USAGE:
    rust-gopher-friend-cli completion <shell>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <shell>    Shell type [possible values: bash, fish, zsh, powershell, elvish]
```

* Checking with an incorrect parameter value:
```bash
cargo run -- completion foo
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-gopher-friend-cli completion foo`
error: 'foo' isn't a valid value for '<shell>'
	[possible values: bash, elvish, fish, powershell, zsh]


USAGE:
    rust-gopher-friend-cli completion <shell>

For more information try --help
```

* Running with the correct parameter value:
```bash
cargo run -- completion bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust-gopher-friend-cli completion bash`
_rust-gopher-friend-cli() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            rust-gopher-friend-cli)
                cmd="rust-gopher-friend-cli"
                ;;
...
...
```
* Check completion script is working as expected.

1. Extend your PATH with the program location.
2. Make sure the program is in your path.
3. Source the completion script.
3. Enter program name and hit tab to see if the completion is working.

```bash
1  export PATH=$PATH:$PWD/target/debug

2  rust-gopher-friend-cli
rust-gopher-friend-cli 0.1.0
Gopher CLI application written in Rust.

USAGE:
    rust-gopher-friend-cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    completion    Generate completion script
    get           This command will get the desired Gopher
    help          Prints this message or the help of the given subcommand(s)

3  source <(rust-gopher-friend-cli completion bash)

4  rust-gopher-friend-cli
completion  get         -h          help        --help      -V          --version
```

Completion is working like a charm !

### Finalize the get command

Here we need a http client to retrieve our gopher from Aurelie's site.
There are multiple options a famous one of them is the request crater.
However it is using async io and maybe in our case it is a bit more complex. So I decided to use the minreq crate, which is a bit lighter and use synchronous blocking io.

#### Add the minreq dependency
So as before we can add it to Cargo.toml or use the Cargo add command.
Anyway at the end, we should have this is the Cargo.toml:
```toml
[dependencies]
minreq = "2.4.2"
structopt = "0.3.22"
```

#### So we end up with this code.
```rust
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
```
#### Let's explain this block code step by step
* We import the File struct from std::fs and Write trait from std::io
```rust
use std::fs::File;
use std::io::Write;
```
* In main we call the get_gopher() function for the get command.
```rust
        Command::Get { gopher } => get_gopher(gopher),
```

* We define the get_gopher() function. The function will take a parameter as a string which is the get value specified on the command line. It may content the gopher file name we want to retrieve.
```rust
fn get_gopher(gopher: String) {
    println!("Try to get {} Gopher...", gopher);
```

* We craft the url, and use the get() method from minreq to have a http response that should contain our gopher file.
* If something really bad happen, then we panic out with an error message thanks to the expect() method.
```rust
    let url = format!("https://github.com/scraly/gophers/raw/main/{}.png", gopher);
    let response = minreq::get(url)
        .send()
        .expect("Fail to get response from server");
```
* Finally we check if our reponse status is 200.
* If yes, we create a local file and write the response bytes to it, using the write_all() method. If something goes wrong with the file creation or write operation we panic out with an error message.
* If not, we just display an error message.
```rust
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
```

#### Let's check if it works.

```bash
cargo run -- get star-wars
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-gopher-friend-cli get star-wars`
Try to get star-wars Gopher...
thread 'main' panicked at 'Fail to get response from server: HttpsFeatureNotEnabled', src/main.rs:29:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

And it's a fail !!!
We fail on this line:
```rust
    let response = minreq::get(url)
        .send()
        .expect("Fail to get response from server");
```

`HttpsFeatureNotEnabled` warn us that something is missing to manage https connections.
And yes, some crate like minreq have optional features. This makes them lighter, speed up compilation time or let us choose various option.
As an example minreq can use the tls library openssl (C based) or rustls (rust based) to manage tls connections.

So looking at the documentation we need to add the feature we like. Here I choose `https-rustls-probe` to use the rustls library.

We need to update Cargo.toml with the feature like this.
```toml
[dependencies]
minreq = { version = "2.4.2", features = ["https-rustls-probe"] }
structopt = "0.3.22"
```

Let's run again.
```bash
cargo run -- get star-wars
   Compiling rust-gopher-friend-cli v0.1.0 (/home/uggla/workspace/rust/rust-gopher-friend-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 1.24s
     Running `target/debug/rust-gopher-friend-cli get star-wars`
Try to get star-wars Gopher...
Perfect! Just saved in star-wars.png
 ll
.rw-rw-r-- uggla uggla  17 KB Sun Aug  1 18:52:04 2021  article.md
.rw-rw-r-- uggla uggla  12 KB Sun Aug  1 18:52:53 2021  Cargo.lock
.rw-rw-r-- uggla uggla 179 B  Sun Aug  1 18:52:53 2021  Cargo.toml
drwxrwxr-x uggla uggla 4.0 KB Sun Aug  1 17:57:50 2021  src
.rw-rw-r-- uggla uggla 903 KB Sun Aug  1 18:53:02 2021  star-wars.png
.rw-rw-r-- uggla uggla 891 KB Wed Jul 28 23:40:41 2021  stargate.png
drwxrwxr-x uggla uggla 4.0 KB Wed Jul 28 20:02:57 2021  target

 gthumb star-wars.png
```
![Our postcard from Go gopher](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/pqg7u1uzfqz2wgmhvqar.png)


### Personal conclusion
Coding this small example was not difficult using Rust. It was pretty straight forward without real difficulties.
To my mind the resulting code is really concise and expressive so not hard to read.
However you need to explore a bit more the crates as Rust core developers try to keep only essential stuff in the standard library.
The Rust ecosystem tend to become really mature and the crate quality is good to really good.
I think developing cli program in Rust is really accessible and a good way to start and progress with this language.

We are done for this article.
Our errors are not well managed yet. It would be better to bubble up the errors and manage them in the main program function.
This will be explained in the next article.

All the code is available on my [github](https://github.com/uggla/rust-gopher-friend-cli) account and branches are used to describe the main steps.

Please let me know if you enjoy this article in the comments or on twitter.

See ya.

