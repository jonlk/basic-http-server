use std::fmt::Error;

use anyhow::{anyhow, Result};

pub fn process_request(id: u32) -> Result<String> {
    let success: bool = true;

    //some processing logic goes here

    if success == true {
        Ok(std::fmt::format(format_args!(
            "processed request for id: {:?}",
            id
        )))
    } else {
        Err(anyhow!("nothing found for id: {:?}", id))
    }
}
