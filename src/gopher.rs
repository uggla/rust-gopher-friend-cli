use std::fs::File;
use std::io::Write;

use crate::BASE_URL;

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
