//! Simple HTTPS GET
//!
//! This example is a Rust adaptation of the [C example of the same
//! name](https://curl.se/libcurl/c/https.html).

extern crate curl;

use curl::easy::Easy;
use std::io::{stdout, Write};

fn main() -> Result<(), curl::Error> {
    let mut curl = Easy::new();

    curl.url("https://crates.io/api/v1/crates/thread_local/1.1.3/download")?;
    curl.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;

    curl.perform()?;

    Ok(())
}
