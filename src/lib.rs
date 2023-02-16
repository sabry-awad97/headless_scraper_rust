// use headless_chrome::{protocol::cdp::Page, Browser, LaunchOptionsBuilder, Tab};
// use std::error::Error;
// use std::fs::write;
// use std::sync::Arc;

// pub enum ExamplePage {
//     HomePage,
//     AboutPage,
//     ContactPage,
// }

// impl ExamplePage {
//     pub fn url(&self) -> &str {
//         match self {
//             ExamplePage::HomePage => "https://www.example.com",
//             ExamplePage::AboutPage => "https://www.example.com/about",
//             ExamplePage::ContactPage => "https://www.example.com/contact",
//         }
//     }
// }

// pub struct ExampleBrowser {
//     browser: Browser,
//     tab: Arc<Tab>,
// }

// impl ExampleBrowser {
//     pub fn new() -> Result<Self, Box<dyn Error>> {
//         let browser = Browser::new(LaunchOptionsBuilder::default().headless(true).build()?)?;

//         let tab = browser.wait_for_initial_tab()?;

//         Ok(Self { browser, tab })
//     }

//     pub fn navigate_to(&self, page: &ExamplePage) -> Result<(), Box<dyn Error>> {
//         self.tab.navigate_to(page.url())?;
//         self.tab.wait_until_navigated()?;
//         Ok(())
//     }

//     pub fn get_title(&self) -> Result<String, Box<dyn Error>> {
//         self.tab.get_title().map_err(|e| e.into())
//     }

//     pub fn get_url(&self) -> Result<String, Box<dyn Error>> {
//         Ok(self.tab.get_url())
//     }

//     pub fn get_html(&self) -> Result<String, Box<dyn Error>> {
//         self.tab.get_content().map_err(|e| e.into())
//     }

//     pub fn screenshot(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let jpeg_data = self.tab.capture_screenshot(
//             Page::CaptureScreenshotFormatOption::Jpeg,
//             None,
//             None,
//             true,
//         )?;

//         write(path, &jpeg_data)?;

//         Ok(())
//     }

//     pub fn fill_form(
//         &self,
//         form_data: &[(String, String)],
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let form = self.tab.wait_for_element("form")?;

//         for (name, value) in form_data {
//             let input = form.find_element(&format!("input[name='{}']", name))?;
//             input.click()?;
//             input.parent.type_str(value)?;
//         }

//         Ok(())
//     }

//     pub fn click_link(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let link = self
//             .tab
//             .wait_for_element(&format!("a:contains('{}')", text))?;
//         link.click()?;
//         self.tab.wait_until_navigated()?;
//         Ok(())
//     }

//     pub fn wait_for_element_visible(
//         &self,
//         selector: &str,
//         timeout: u64,
//     ) -> Result<bool, Box<dyn std::error::Error>> {
//         let start_time = std::time::Instant::now();

//         loop {
//             match self.tab.find_element(selector) {
//                 Ok(element) => {
//                     let box_model = element.get_box_model()?;
//                     if box_model.width == 0.0 || box_model.height == 0.0 {
//                         return Ok(false);
//                     }
//                 }

//                 _ => {
//                     if start_time.elapsed().as_secs() > timeout {
//                         {
//                             return Err("Timeout waiting for element to become visible".into());
//                         }
//                     }
//                 }
//             }

//             std::thread::sleep(std::time::Duration::from_millis(100));
//         }
//     }

//     pub fn reload(&self) -> Result<(), Box<dyn std::error::Error>> {
//         self.tab.reload(true, None)?;
//         self.tab.wait_until_navigated()?;
//         Ok(())
//     }

    
// }

// // implement more advanced methods