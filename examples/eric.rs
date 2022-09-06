//! Simple HTTPS GET
//!
//! This example is a Rust adaptation of the [C example of the same
//! name](https://curl.se/libcurl/c/https.html).

extern crate curl;

use curl::easy::{HttpVersion, Easy};

fn main() -> Result<(), curl::Error> {
    let mut curl = Easy::new();

    curl.url("https://crates.io/api/v1/crates/thread_local/1.1.3/download")?;
    curl.write_function(|data| {
        eprintln!("Got {} bytes", data.len());
        Ok(data.len())
    })?;
    curl.useragent(&format!("cargo 1.65.0"))?;
    curl.follow_location(true)?;
    curl.http_version(HttpVersion::V2)?;
    curl.pipewait(true)?;
    curl.perform()?;
    let code = curl.response_code()?;
    eprintln!("finished with {:?}", code);

    Ok(())
}
