use headless_chrome::Browser;
use headless_chrome::{protocol::cdp::Page, LaunchOptionsBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::new(LaunchOptionsBuilder::default().headless(false).build()?)
        .map_err(|e| format!("Failed to launch browser: {}", e))?;

    let tab = browser
        .wait_for_initial_tab()
        .map_err(|e| format!("Failed to get initial tab: {}", e))?;

    let url = "https://www.tidalhair.com/products/tidal-10-minute-hair-mask?variant=39661967605927";
    if let Err(e) = tab.navigate_to(url) {
        eprintln!("Failed to navigate to {}: {}", url, e);
    }

    if let Err(e) = tab.wait_until_navigated() {
        eprintln!("Failed while waiting for navigation: {}", e);
    }

    println!("Navigating to {}...", tab.get_url());

    if let Err(e) = tab.get_title() {
        eprintln!("Failed to get page title: {}", e);
    }

    let html_path = "example.html";
    println!("Saving HTML to {}...", html_path);
    if let Ok(html) = tab.get_content() {
        if let Err(e) = fs::write(html_path, html) {
            eprintln!("Failed to save HTML to file: {}", e);
        }
    } else {
        eprintln!("Failed to get page content");
    }

    let screenshot_path = "example.png";
    println!("Taking screenshot and saving to {}...", screenshot_path);
    if let Ok(jpeg_data) =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)
    {
        if let Err(e) = fs::write(screenshot_path, &jpeg_data) {
            eprintln!("Failed to save screenshot to file: {}", e);
        }
    } else {
        eprintln!("Failed to capture screenshot");
    }

    let button_selector = ".oke-showMore-button-text.oke-button-text";
    let main_selector = ".oke-w-reviews-list-item";
    let field_names = vec!["title", "text", "date", "name"];
    let selectors = vec![
        ".oke-reviewContent-title.oke-title",
        ".oke-reviewContent-body.oke-bodyText",
        ".oke-reviewContent-date",
        ".oke-w-reviewer-name",
    ];

    loop {
        if let Ok(button_element) = tab.wait_for_element(button_selector) {
            if let Err(e) = button_element.click() {
                eprintln!("Failed to click on element: {}", e);
            }

            let parents = tab.find_elements(main_selector)?;
            for parent in parents {
                let mut element_map: HashMap<&str, Vec<String>> = HashMap::new();
                
                for (field_name, selector) in field_names.iter().zip(selectors.iter()) {
                    if let Ok(child) = parent.find_elements(selector) {
                        for el in child {
                            if let Ok(text) = el.get_inner_text() {
                                element_map.entry(field_name).or_default().push(text);
                            }
                        }
                    }
                }
                
                println!("{:?}", element_map);
            }
        } else {
            eprintln!("Failed to find element with selector '{}'", button_selector);
            break;
        }
    }

    Ok(())
}
