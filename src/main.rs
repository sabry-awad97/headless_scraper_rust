use headless_chrome::Browser;
use headless_chrome::{protocol::cdp::Page, LaunchOptionsBuilder};
use std::error::Error;
use std::io::Write;
fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::new(LaunchOptionsBuilder::default().headless(false).build()?)?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(
        "https://www.tidalhair.com/products/tidal-10-minute-hair-mask?variant=39661967605927",
    )?;

    tab.wait_until_navigated()?;

    println!("Navigating to {}...", tab.get_url());

    println!("Title: {}", tab.get_title()?);

    let html_path = "example.html";
    println!("Saving HTML to {}...", html_path);
    let html = tab.get_content()?;
    std::fs::write(html_path, html)?;

    let screenshot_path = "example.png";
    println!("Taking screenshot and saving to {}...", screenshot_path);
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    let mut file = std::fs::File::create(screenshot_path).unwrap();
    file.write_all(&jpeg_data).unwrap();

    let button_selector = ".oke-showMore-button-text.oke-button-text";

    loop {
        let button_element = tab.wait_for_element(button_selector)?;
        let box_model = button_element.get_box_model()?;
        println!("width {}, height {}", box_model.width, box_model.height);
        button_element.click()?;
    }
}
