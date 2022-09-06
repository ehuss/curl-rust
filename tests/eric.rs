use curl::easy::{HttpVersion, Easy};

#[test]
fn basic() -> Result<(), curl::Error> {
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

#[test]
fn basic_no_pipewait() -> Result<(), curl::Error> {
    let mut curl = Easy::new();

    curl.url("https://crates.io/api/v1/crates/thread_local/1.1.3/download")?;
    curl.write_function(|data| {
        eprintln!("Got {} bytes", data.len());
        Ok(data.len())
    })?;
    curl.useragent(&format!("cargo 1.65.0"))?;
    curl.follow_location(true)?;
    curl.http_version(HttpVersion::V2)?;
    curl.perform()?;
    let code = curl.response_code()?;
    eprintln!("finished with {:?}", code);

    Ok(())
}

#[test]
fn basic_no_v2() -> Result<(), curl::Error> {
    let mut curl = Easy::new();

    curl.url("https://crates.io/api/v1/crates/thread_local/1.1.3/download")?;
    curl.write_function(|data| {
        eprintln!("Got {} bytes", data.len());
        Ok(data.len())
    })?;
    curl.useragent(&format!("cargo 1.65.0"))?;
    curl.follow_location(true)?;
    curl.perform()?;
    let code = curl.response_code()?;
    eprintln!("finished with {:?}", code);

    Ok(())
}
