use headless_scraper::{ExampleBrowser, ExamplePage};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let browser = ExampleBrowser::new()?;
    let page = ExamplePage::HomePage;
    browser.navigate_to(&page)?;
    println!("Page title: {}", browser.get_title()?);
    Ok(())
}
