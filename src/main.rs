use csv::Writer;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{Browser, Tab};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use scraper::{Html, Selector};

#[derive(Serialize, Debug)]
struct Review {
    title: String,
    text: String,
    date: String,
    name: String,
}

impl Review {
    fn from_map(map: HashMap<&str, String>) -> Review {
        Review {
            title: map.get("title").unwrap().to_owned(),
            text: map.get("text").unwrap().to_owned(),
            date: map.get("date").unwrap().to_owned(),
            name: map.get("name").unwrap().to_owned(),
        }
    }
}

#[derive(Debug)]
enum FindError {
    ElementNotFound,
    ClickError,
}

impl std::fmt::Display for FindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindError::ElementNotFound => write!(f, "Element not found"),
            FindError::ClickError => write!(f, "Error while clicking on element"),
        }
    }
}

impl Error for FindError {}

fn get_reviews(
    tab: Arc<Tab>,
    page_reviews: Option<usize>,
    max_reviews: Option<usize>,
) -> Result<Vec<Review>, Box<dyn Error>> {
    let mut reviews = Vec::new();

    let button_css_selector = ".oke-showMore-button-text.oke-button-text";
    let main_selector = Selector::parse(".oke-w-reviews-list-item").unwrap();
    let field_selectors = [
        (Selector::parse(".oke-reviewContent-title.oke-title").unwrap(), "title"),
        (Selector::parse(".oke-reviewContent-body.oke-bodyText").unwrap(), "text"),
        (Selector::parse(".oke-reviewContent-date").unwrap(), "date"),
        (Selector::parse(".oke-w-reviewer-name").unwrap(), "name"),
    ];

    let mut previous_length = 0;
    let mut page_number = 1;
    loop {
        let html = tab.get_content()?;
        let document = Html::parse_document(html.as_str());
        let parents = document.select(&main_selector).skip(previous_length).collect::<Vec<_>>();
        for parent in parents {
            let mut element_map = HashMap::new();
            for (field_selector, field_name) in field_selectors.iter() {
                let child = parent.select(&field_selector).next().ok_or_else(|| FindError::ElementNotFound)?;
                let value = child.text().collect::<Vec<_>>().join(", ");
                element_map.insert(*field_name, value);
            }
            reviews.push(Review::from_map(element_map));
            previous_length = reviews.len();
        }

        if let Some(max_reviews) = max_reviews {
            if reviews.len() >= max_reviews {
                return Ok(reviews);
            }
        }

        if let Some(page_reviews) = page_reviews {
            if previous_length % page_reviews != 0 {
                break;
            }
        }

        let button_element = match tab.wait_for_element(button_css_selector) {
            Ok(elem) => elem,
            Err(_) => break,
        };

        if let Err(_) = button_element.click() {
            return Err(Box::new(FindError::ClickError));
        }

        page_number += 1;

        println!("{}", page_number);
    }

    Ok(reviews)
}

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

    let reviews = get_reviews(tab, None, None)?;

    let mut csv_writer = Writer::from_path("reviews.csv")?;

    for review in &reviews {
        csv_writer.serialize(review)?;
    }

    csv_writer.flush()?;

    Ok(())
}
