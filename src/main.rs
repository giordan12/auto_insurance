// Plan:
// We will use thirtyfour to load the website, collect the html and send that to gpt
// If possible we will also send a screenshot
// Then we will prompt the model to only return ids of the elements and see if that
// option works fine.

use std::error::Error;
use std::fs::File;
use std::io::Write;
use thirtyfour::prelude::*;

pub mod openai_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto("https://statefarm.com").await?;
    let _ = driver.query(By::Id("oneX-main-content")).first().await; // We are waiting for the page to finish loading, might have to wait for something else

    // Let's get the webpage source and store it in a var for now
    let page_source = driver
        .source()
        .await
        .expect("Failed to pull the source from the page");

    print!("Successfully retrieved the source of the page, will proceed to save it now");

    let mut file = File::create("temp_file.html")?;
    file.write_all(page_source.as_bytes()); // TODO remove this, this is just to test

    // TODO figure out how to call the open ai api (make a small library around that using traits and send the collected html)
    // see if attaching a screenshot of the page can help the model figure out what it does
    driver.quit().await?;
    Ok(())
}
