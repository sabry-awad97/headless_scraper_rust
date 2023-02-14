use headless_scraper::{ExampleBrowser, ExamplePage};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let browser = ExampleBrowser::new()?;
    let page = ExamplePage::HomePage;

    println!("Navigating to {}...", page.url());
    browser.navigate_to(&page)?;

    println!("Title: {}", browser.get_title()?);
    println!("URL: {}", browser.get_url()?);

    let html_path = "example.html";
    println!("Saving HTML to {}...", html_path);
    let html = browser.get_html()?;
    std::fs::write(html_path, html)?;

    let screenshot_path = "example.png";
    println!("Taking screenshot and saving to {}...", screenshot_path);
    browser.screenshot(screenshot_path)?;
    Ok(())
}
