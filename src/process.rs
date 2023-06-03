use std::fmt::Error;

use anyhow::{anyhow, Result};

pub fn process_request(id: u32) -> Result<u32> {
    let success: bool = if id == 35 { true } else { false };
    if success == true {
        Ok(id)
    } else {
        Err(anyhow!("nothing found for id: {:?}", id))
    }
}
