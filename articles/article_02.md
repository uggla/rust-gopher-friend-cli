## Why this article

This article is {% post https://dev.to/uggla/rust-cli-example-ferris-fetches-go-gopher-postcards-3gb5 %}

second part.

Before starting, I would like to thank everybody who read the previous article. The statistics were encouraging for a first article and fuel my motivation to continue writing new posts.

The article was also mentioned in [This week in rust](https://this-week-in-rust.org/) newsletter. So a big thank you to the authors. I can only highly recommend this newsletter as collecting all information about the rust ecosystem, every week, is an amazing job.

## Status and goals

In the previous post, we ended with a working CLI. However, the error management was not well implemented. In most of the case, we want to bubble up the errors and deal with them in the main program function.

So this is what we will do now. Then we will improve the code a bit, separate it into files and add a logger.

## Use our errors

We will focus on the get_gopher() function and explain the changes step by step.
The compiler will be our assistant and will help us to resolve errors.

```rust
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
```

Our first goal will be to refactor the code and change the function signature to return a Result.

```rust
fn get_gopher(gopher: String) -> Result<String, Error> {
...
}
```

Of course, at this point, the compiler will yell because the type Error is not defined.
```rust
cannot find type `Error` in this scope: not found in this scope
```

So we need to define it with an enum.

```rust
enum Error {
    GopherNotFound(String),
}
```

Now the compiler will yell again because our function does not return a Result.

```rust
error[E0308]: mismatched types
  --> src/main.rs:35:36
   |
35 |       if response.status_code == 200 {
   |  ____________________________________^
36 | |         let file_name = format!("{}.png", gopher);
37 | |         let mut output_file = File::create(&file_name).expect("Fail to create file");
38 | |         output_file
...  |
41 | |         println!("Perfect! Just saved in {}", &file_name);
42 | |     } else {
   | |_____^ expected enum `Result`, found `()`
   |
   = note:   expected enum `Result<String, Error>`
           found unit type `()`
...
```

So let's do it.
In the first branch of the if, we implicitly (no semicolon at the end of the line) return a string
```rust
Ok(format!("Perfect! Just saved in {}", &file_name))
```
and in the other, we return an Error::GopherNotFound with a description string.
```rust
Err(Error::GopherNotFound(format!(
            "Gopher {} not exists",
            gopher
        )))
```

So we ended up with the following function.

```rust
fn get_gopher(gopher: String) -> Result<String, Error> {
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
        Ok(format!("Perfect! Just saved in {}", &file_name))
    } else {
        Err(Error::GopherNotFound(format!(
            "Gopher {} not exists",
            gopher
        )))
    }
}
```

The function looks ok, but the compiler is still not happy.
```rust
error[E0308]: `match` arms have incompatible types
  --> src/main.rs:55:13
   |
52 | /     match cmd {
53 | |         Command::Get { gopher } => get_gopher(gopher),
   | |                                    ------------------ this is found to be of type `Result<String, Error>`
54 | |         Command::Completion { shell } => {
55 | |             Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
   | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found `()`
56 | |         }
57 | |     }
   | |_____- `match` arms have incompatible types
   |
   = note:   expected type `Result<String, Error>`
           found unit type `()`
...
```
As we change the definition of our function to return a Result, we also need to change the call in the main function and manage the Result.
This can be simply done using pattern matching around the get_gopher() function.

```rust
fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => println!("{}", msg),
            Err(Error::GopherNotFound(msg)) => eprintln!("{}", msg),
        },
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}
```
And now the program can compile without errors or warnings.
So we have defined our error if our gopher is not available. But the get_gother() function can still fail and panic. As an example, if the file cannot be created. In fact, we need to remove all the expect() methods that could make our code panic in the get_gopher() function.
This can be done using the question mark operator.
Let's refactor our code and remove the first expect() of the function.

So we change the following code from:
```rust
    let response = minreq::get(url)
        .send()
        .expect("Fail to get response from server");

```
to:

```rust
    let response = minreq::get(url).send()?;
```

Unfortunately, the compiler is not happy again.
```rust
error[E0277]: `?` couldn't convert the error to `Error`
  --> src/main.rs:31:43
   |
28 | fn get_gopher(gopher: String) -> Result<String, Error> {
   |                                  --------------------- expected `Error` because of this
...
31 |     let response = minreq::get(url).send()?;
   |                                           ^ the trait `From<minreq::Error>` is not implemented for `Error`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, minreq::Error>>` for `Result<String, Error>`
   = note: required by `from_residual`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
```

However, it gives us what is wrong and explains how to fix the issue. If the send() method fails, it returns a minreq::Error, and our function expects an Error. So we need a conversion. It can be achieved by implementing the From trait.
So let's do that, first, we need to add this new kind of error (Response) in our Error enum.

```rust
enum Error {
    GopherNotFound(String),
    Response(String),
}
```

And now we implement the conversion with the From trait:
```rust
impl From<minreq::Error> for Error {
    fn from(err: minreq::Error) -> Self {
        Error::Response(err.to_string())
    }
}
```

```rust
error[E0004]: non-exhaustive patterns: `Err(Response(_))` not covered
   --> src/main.rs:58:42
    |
58  |         Command::Get { gopher } => match get_gopher(gopher) {
    |                                          ^^^^^^^^^^^^^^^^^^ pattern `Err(Response(_))` not covered
    | 
   ::: /home/uggla/rust/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:250:5
    |
250 |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
    |     --- not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Result<String, Error>`
```

Rust is really strict, and the pattern matching needs to cover all cases. As we have just introduced a new case (Error::Response), we need to also modify the main function in such way.

```rust
fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => println!("{}", msg),
            Err(Error::GopherNotFound(msg)) => eprintln!("{}", msg),
            Err(Error::Response(msg)) => eprintln!("{}", msg),
        },
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}
```

Now the compiler is happy and we don't have errors or warnings anymore.
We can now get rid of the next expect() and proceed exactly like we just have done before.

```rust
fn get_gopher(gopher: String) -> Result<String, Error> {
    println!("Try to get {} Gopher...", gopher);
    let url = format!("https://github.com/scraly/gophers/raw/main/{}.png", gopher);
    let response = minreq::get(url).send()?;

    if response.status_code == 200 {
        let file_name = format!("{}.png", gopher);
        let mut output_file = File::create(&file_name)?;
        output_file
            .write_all(response.as_bytes())
            .expect("Fail to write file");
        Ok(format!("Perfect! Just saved in {}", &file_name))
    } else {
        Err(Error::GopherNotFound(format!(
            "Gopher {} not exists",
            gopher
        )))
    }
}
```

Here we have the following error as the std::io::Error type needs to be converted to our Error type.
```rust
error[E0277]: `?` couldn't convert the error to `Error`
  --> src/main.rs:42:55
   |
35 | fn get_gopher(gopher: String) -> Result<String, Error> {
   |                                  --------------------- expected `Error` because of this
...
42 |         let mut output_file = File::create(&file_name)?;
   |                                                       ^ the trait `From<std::io::Error>` is not implemented for `Error`
   |
...
```

Add a new kind of error (IO).
```rust
enum Error {
    GopherNotFound(String),
    Response(String),
    IO(String),
}
```

Implement the conversion.
```rust
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err.to_string())
    }
}
```

Add the case in main()
```rust
fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => println!("{}", msg),
            Err(Error::GopherNotFound(msg)) => eprintln!("{}", msg),
            Err(Error::Response(msg)) => eprintln!("{}", msg),
            Err(Error::IO(msg)) => eprintln!("{}", msg),
        },
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}
```
Looks good, remove the latest expect().
```rust
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
```

As write_all() method returns a std::io::Error in case of failure and, we already implemented this conversion. There is nothing more to do.

## Small refactoring to improve code

### Specify the BASE_URL
It will be more convenient to specify the base URL to retrieve the gophers.
We can simply define a const at the beginning of our program.

```rust
const BASE_URL: &str = "https://github.com/scraly/gophers/raw/main";
```

And craft the URL variable using the BASE_URL const.

```rust
fn get_gopher(gopher: String) -> Result<String, Error> {
    println!("Try to get {} Gopher...", gopher);
    let url = format!("{}/{}.png", BASE_URL, gopher);
    ...
```

### Factorize errors and send an errorlevel to the OS
Create the function
```rust
fn display_error_and_exit(error_msg: String) {
    eprintln!("{}", error_msg);
    exit(255)
}
```

We need to define the exit function.
```rust
use std::process::exit;
```

Call the display_error_and_exit() function from main.
```rust
fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => println!("{}", msg),
            Err(Error::GopherNotFound(msg)) => display_error_and_exit(msg),
            Err(Error::Response(msg)) => display_error_and_exit(msg),
            Err(Error::IO(msg)) => display_error_and_exit(msg),
        },
...
```

## Create a gopher module to separate responsibility into files.

The idea is to move the get_gopher() function into a module.
The benefits will be to:
* Reduce the size of main.
* Better separate things.
* Module will improve code reusability.

First, we need to create a gopher.rs file in our src directory. Then we move the error definitions and the get_gopher() function.
```rust
const BASE_URL: &str = "https://github.com/scraly/gophers/raw/main";

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
    let url = format!("{}/{}.png", BASE_URL, gopher);
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
```

Of course, at this point, the compiler is becoming mad because it can not find the get_gopher() function and errors definitions.
```rust
cannot find function `get_gopher` in this scope: not found in this scope
```

We need to tell it, that it is now in a new module/file.
So let's do it in our main file.
```rust
mod gopher;
```

We need also to import the function from the gopher module.
```rust
use gopher::*;
```

Unfortunately, this is still not working as the get_gopher() function is private. We need to change it to public as well as the enum declaration.
```rust
pub enum Error {
...
pub fn get_gopher(gopher: String) -> Result<String, Error> {
...
```

After saving most of the errors vanished. There are remaining ones about import not used.
```rust
unused import: `std::fs::File`
unused import: `std::io::Write`
```
We simply need to move them in the gopher module.
```rust
use std::fs::File;
use std::io::Write;
```

So we ended up with the following code:
main.rs
```rust
mod gopher;
use gopher::*;

use std::io::stdout;
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

fn display_error_and_exit(error_msg: String) {
    eprintln!("{}", error_msg);
    exit(255)
}

fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Get { gopher } => match get_gopher(gopher) {
            Ok(msg) => println!("{}", msg),
            Err(Error::GopherNotFound(msg)) => display_error_and_exit(msg),
            Err(Error::Response(msg)) => display_error_and_exit(msg),
            Err(Error::IO(msg)) => display_error_and_exit(msg),
        },
        Command::Completion { shell } => {
            Command::clap().gen_completions_to(crate_name!(), shell, &mut stdout())
        }
    }
}

```
gopher.rs:

```rust
use std::fs::File;
use std::io::Write;

const BASE_URL: &str = "https://github.com/scraly/gophers/raw/main";

pub enum Error {
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

pub fn get_gopher(gopher: String) -> Result<String, Error> {
    println!("Try to get {} Gopher...", gopher);
    let url = format!("{}/{}.png", BASE_URL, gopher);
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
```

## Add a simple logger

The idea here is to remove all println!() and eprintln!() macros and use a simple logger to give information to the user.

First, we need to add the required dependencies to our `Cargo.toml`.
We will use the `log` crate as a frontend log facility. Here is an extract of the documentation to understand the purpose of this crate.
*A logging facade provides a single logging API that abstracts over the actual logging implementation. Libraries can use the logging API provided by this crate, and the consumer of those libraries can choose the logging implementation that is most suitable for its use case.*
[Sources](https://github.com/rust-lang/log)


As a backend facility, we will use the `simple_logger` crate. This crate will simply output messages formatted like this `2015-02-24 01:05:20 WARN [logging_example] This is an example message.`
I like the crate because it is simple to use and a good fit for small projects. Also, I contributed to another project (rust-riemann-client) maintained by the same author @borntyping (hello Sam) and, it was really a cool experience.
[Sources](https://github.com/borntyping/rust-simple_logger)


```rust
[dependencies]
minreq = { version = "2.4.2", features = ["https-rustls-probe"] }
structopt = "0.3.22"
log = "0.4.14"
simple_logger = "1.13.0"
```

Now we just need to initialize our simple logger.

We import it.
```rust
use simple_logger::SimpleLogger;
```

And initialize it at the beginning of main with the default level set to info.
```rust
fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
...
```

Now we have just to replace println! and eprintln! macros with the respective ones log::info! and log::error!.
main.rs
```rust
mod gopher;

use gopher::*;
use simple_logger::SimpleLogger;
use std::io::stdout;
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
```
gopher.rs
```rust
use std::fs::File;
use std::io::Write;

const BASE_URL: &str = "https://github.com/scraly/gophers/raw/main";

pub enum Error {
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

pub fn get_gopher(gopher: String) -> Result<String, Error> {
    log::info!("Try to get {} Gopher...", gopher);
    let url = format!("{}/{}.png", BASE_URL, gopher);
    let response = minreq::get(url).send()?;

    if response.status_code == 200 {
        let file_name = format!("{}.png", gopher);
        let mut output_file = File::create(&file_name)?;
        output_file.write_all(response.as_bytes())?;
        Ok(format!("Perfect! Just saved in {}", &file_name))
    } else {
        Err(Error::GopherNotFound(format!(
            "Gopher {} does not exist",
            gopher
        )))
    }
}
```

### Run example
#### Run ok
```bash
 cargo run -- get friends
   Compiling rust-gopher-friend-cli v0.1.0 (/home/uggla/workspace/rust/rust-gopher-friend-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 1.56s
     Running `target/debug/rust-gopher-friend-cli get friends`
2021-09-08 01:00:36,294 INFO [rust_gopher_friend_cli::gopher] Try to get friends Gopher...
2021-09-08 01:00:39,169 INFO [rust_gopher_friend_cli] Perfect! Just saved in friends.png
```

#### Run with error
```bash
 cargo run -- get friendsz
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust-gopher-friend-cli get friendsz`
2021-09-08 01:00:52,245 INFO [rust_gopher_friend_cli::gopher] Try to get friendsz Gopher...
2021-09-08 01:00:52,943 ERROR [rust_gopher_friend_cli] Gopher friendsz does not exist
```

We are done with this article.

All the code is available on my [github](https://github.com/uggla/rust-gopher-friend-cli) account and branches are used to describe the main steps.

Please let me know if you enjoy this article in the comments or on Twitter.

See ya.
