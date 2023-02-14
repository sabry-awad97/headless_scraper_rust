use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::sync::Arc;

pub enum ExamplePage {
    HomePage,
    AboutPage,
    ContactPage,
}

impl ExamplePage {
    fn url(&self) -> &str {
        match self {
            ExamplePage::HomePage => "https://www.example.com",
            ExamplePage::AboutPage => "https://www.example.com/about",
            ExamplePage::ContactPage => "https://www.example.com/contact",
        }
    }
}

pub struct ExampleBrowser {
    browser: Browser,
    tab: Arc<Tab>,
}

impl ExampleBrowser {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let browser = Browser::new(LaunchOptionsBuilder::default().headless(true).build()?)?;

        let tab = browser.wait_for_initial_tab()?;

        Ok(Self { browser, tab })
    }

    pub fn navigate_to(&self, page: &ExamplePage) -> Result<(), Box<dyn std::error::Error>> {
        self.tab.navigate_to(page.url())?;
        self.tab.wait_until_navigated()?;
        Ok(())
    }

    pub fn get_title(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.tab.get_title().map_err(|e| e.into())
    }
}
