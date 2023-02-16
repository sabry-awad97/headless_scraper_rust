use headless_chrome::Browser;
use headless_chrome::{protocol::cdp::Page, LaunchOptionsBuilder};
use std::error::Error;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::new(LaunchOptionsBuilder::default().headless(false).build()?)
        .map_err(|e| format!("Failed to launch browser: {}", e))?;

    let tab = browser.wait_for_initial_tab()
        .map_err(|e| format!("Failed to get initial tab: {}", e))?;

    let url = "https://www.tidalhair.com/products/tidal-10-minute-hair-mask?variant=39661967605927";
    tab.navigate_to(url)
        .map_err(|e| format!("Failed to navigate to {}: {}", url, e))?;

    tab.wait_until_navigated()
        .map_err(|e| format!("Failed while waiting for navigation: {}", e))?;

    println!("Navigating to {}...", tab.get_url());

    println!("Title: {}", tab.get_title()
        .map_err(|e| format!("Failed to get page title: {}", e))?);

    let html_path = "example.html";
    println!("Saving HTML to {}...", html_path);
    let html = tab.get_content()
        .map_err(|e| format!("Failed to get page content: {}", e))?;
    fs::write(html_path, html)
        .map_err(|e| format!("Failed to save HTML to file: {}", e))?;

    let screenshot_path = "example.png";
    println!("Taking screenshot and saving to {}...", screenshot_path);
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)
        .map_err(|e| format!("Failed to capture screenshot: {}", e))?;

    let mut file = fs::File::create(screenshot_path)
        .map_err(|e| format!("Failed to create file for screenshot: {}", e))?;
    file.write_all(&jpeg_data)
        .map_err(|e| format!("Failed to save screenshot to file: {}", e))?;

    let button_selector = ".oke-showMore-button-text.oke-button-text";

    loop {
        let button_element = tab.wait_for_element(button_selector)
            .map_err(|e| format!("Failed to find element with selector '{}': {}", button_selector, e))?;
        let box_model = button_element.get_box_model()
            .map_err(|e| format!("Failed to get box model for element: {}", e))?;
        println!("width {}, height {}", box_model.width, box_model.height);
        button_element.click()
            .map_err(|e| format!("Failed to click on element: {}", e))?;
    }
}
