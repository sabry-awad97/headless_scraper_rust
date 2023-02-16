use csv::Writer;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{Browser, Tab};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize, Debug)]
struct Review {
    title: String,
    text: String,
    date: String,
    name: String,
}

impl Review {
    fn from_map(map: HashMap<&str, Vec<String>>) -> Review {
        Review {
            title: map.get("title").unwrap().join(", ").to_owned(),
            text: map.get("text").unwrap().join(", ").to_owned(),
            date: map.get("date").unwrap().join(", ").to_owned(),
            name: map.get("name").unwrap().join(", ").to_owned(),
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

enum Selectors {
    ButtonSelector,
    MainSelector,
    FieldSelector(&'static str, &'static str),
}

impl Selectors {
    fn selector(&self) -> &'static str {
        match self {
            Selectors::ButtonSelector => ".oke-showMore-button-text.oke-button-text",
            Selectors::MainSelector => ".oke-w-reviews-list-item",
            Selectors::FieldSelector(_, selector) => selector,
        }
    }

    fn field_name(&self) -> Option<&'static str> {
        match self {
            Selectors::FieldSelector(field_name, _) => Some(field_name),
            _ => None,
        }
    }
}

fn get_reviews(tab: Arc<Tab>) -> Result<Vec<Review>, Box<dyn Error>> {
    let mut reviews = Vec::new();

    let button_selector = Selectors::ButtonSelector.selector();
    let main_selector = Selectors::MainSelector.selector();
    let field_selectors = [
        Selectors::FieldSelector("title", ".oke-reviewContent-title.oke-title"),
        Selectors::FieldSelector("text", ".oke-reviewContent-body.oke-bodyText"),
        Selectors::FieldSelector("date", ".oke-reviewContent-date"),
        Selectors::FieldSelector("name", ".oke-w-reviewer-name"),
    ];

    let mut previous_length = 0;
    loop {
        let parents = tab.find_elements(main_selector)?;
        for parent in &parents[previous_length..] {
            let mut element_map: HashMap<&str, Vec<String>> = HashMap::new();
            for field_selector in field_selectors.iter() {
                let child = parent.find_elements(field_selector.selector())?;
                let values: Vec<String> = child
                    .iter()
                    .map(|element| element.get_inner_text().unwrap_or_default())
                    .collect();
                element_map.insert(field_selector.field_name().unwrap_or_default(), values);
            }
            reviews.push(Review::from_map(element_map));
            previous_length = reviews.len();
        }
        println!("{}", previous_length);

        if previous_length == 10 {
            break;
        }

        let button_element = match tab.wait_for_element(button_selector) {
            Ok(elem) => elem,
            Err(_) => break,
        };

        if let Err(_) = button_element.click() {
            return Err(Box::new(FindError::ClickError));
        }
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

    let reviews = get_reviews(tab)?;

    let mut csv_writer = Writer::from_path("reviews.csv")?;

    for review in &reviews {
        csv_writer.serialize(review)?;
    }

    csv_writer.flush()?;

    Ok(())
}
