use csv::Writer;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{Browser, Tab};
use scraper::{Html, Selector};
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug)]
struct Review {
    title: String,
    text: String,
    date: String,
    name: String,
}

impl Review {
    fn from_map(map: HashMap<FieldType, String>) -> Review {
        Review {
            title: map[&FieldType::Title].to_owned(),
            text: map[&FieldType::Text].to_owned(),
            date: map[&FieldType::Date].to_owned(),
            name: map[&FieldType::Name].to_owned(),
        }
    }
}

impl Serialize for Review {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Review", 4)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("text", &self.text)?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum FieldType {
    Title,
    Text,
    Date,
    Name,
}

trait ToFieldType {
    fn to_field_type(&self) -> FieldType;
}

impl ToFieldType for &str {
    fn to_field_type(&self) -> FieldType {
        match *self {
            "title" => FieldType::Title,
            "text" => FieldType::Text,
            "date" => FieldType::Date,
            "name" => FieldType::Name,
            _ => panic!("Invalid field type"),
        }
    }
}

#[derive(Debug)]
enum FindError {
    ElementNotFound,
}

struct Field {
    selector: Selector,
    name: FieldType,
}

impl Field {
    fn new(selector: Selector, name: FieldType) -> Field {
        Field { selector, name }
    }

    fn extract_value(&self, parent: scraper::ElementRef<'_>) -> Result<String, FindError> {
        let child = parent
            .select(&self.selector)
            .next()
            .ok_or_else(|| FindError::ElementNotFound)?;

        Ok(child.text().collect::<Vec<_>>().join(", "))
    }
}

impl std::fmt::Display for FindError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindError::ElementNotFound => write!(f, "Element not found"),
        }
    }
}

impl Error for FindError {}

trait ExtractReviews {
    fn extract_reviews(
        &self,
        fields: &[Field],
        page_reviews: Option<usize>,
        max_reviews: Option<usize>,
    ) -> Result<Vec<Review>, Box<dyn Error>>;
}

impl ExtractReviews for Arc<Tab> {
    fn extract_reviews(
        &self,
        fields: &[Field],
        page_reviews: Option<usize>,
        max_reviews: Option<usize>,
    ) -> Result<Vec<Review>, Box<dyn Error>> {
        let start_time = Instant::now();
        let mut reviews = Vec::new();

        let button_css_selector = ".oke-showMore-button-text.oke-button-text";
        let main_selector = Selector::parse(".oke-w-reviews-list-item").unwrap();

        let mut previous_length = 0;
        let mut page_number = 0;

        loop {
            let start_iteration_time = Instant::now();
            let mut current_length = reviews.len();

            if let Some(max_reviews) = max_reviews {
                if current_length >= max_reviews {
                    return Ok(reviews);
                }
            }

            if let Some(page_reviews) = page_reviews {
                if previous_length % page_reviews != 0 {
                    break;
                }
            }

            // Extract reviews on the page
            let html = self.get_content()?;
            let document = Html::parse_document(&html);
            let review_elements = document.select(&main_selector);
            for element in review_elements.skip(previous_length) {
                let mut field_values = HashMap::new();

                for field in fields {
                    let value = field.extract_value(element)?;
                    field_values.insert(field.name, value);
                }

                reviews.push(Review::from_map(field_values));
                current_length = reviews.len();

                // Stop if we've extracted the maximum number of reviews
                if let Some(max) = max_reviews {
                    if current_length >= max {
                        break;
                    }
                }
            }

            if current_length == previous_length {
                println!("No new reviews found in the last iteration");
            } else {
                page_number += 1;
            }

            previous_length = current_length;

            let end_iteration_time = Instant::now();
            let elapsed_iteration_time = end_iteration_time - start_iteration_time;
            let elapsed_time = start_time.elapsed();
            let elapsed_iteration_secs = elapsed_iteration_time.as_secs_f64();
            let elapsed_secs = elapsed_time.as_secs_f64();

            println!(
                "Elapsed time for last iteration: {:.3} seconds",
                elapsed_iteration_secs
            );
            println!("Total elapsed time: {:.3} seconds", elapsed_secs);
            println!("Number of reviews: {}", current_length);
            println!("Page number: {}", page_number);

            match self.wait_for_element(button_css_selector) {
                Ok(element) => {
                    element.click()?;
                }
                Err(_) => break,
            };
        }

        let elapsed = start_time.elapsed().as_secs_f32();
        println!("Extracted {} reviews in {:.2}s", previous_length, elapsed);

        Ok(reviews)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let launch_options = LaunchOptionsBuilder::default().headless(true).build()?;
    let browser = Browser::new(launch_options)?;

    let tab = browser.wait_for_initial_tab()?;

    let url = "https://www.tidalhair.com/products/tidal-10-minute-hair-mask?variant=39661967605927";

    if let Err(e) = tab.navigate_to(url) {
        eprintln!("Failed to navigate to {}: {}", url, e);
    }

    if let Err(e) = tab.wait_until_navigated() {
        eprintln!("Failed while waiting for navigation: {}", e);
    }

    let fields = [
        Field::new(
            Selector::parse(".oke-reviewContent-title.oke-title").unwrap(),
            FieldType::Title,
        ),
        Field::new(
            Selector::parse(".oke-reviewContent-body.oke-bodyText").unwrap(),
            FieldType::Text,
        ),
        Field::new(
            Selector::parse(".oke-reviewContent-date").unwrap(),
            FieldType::Date,
        ),
        Field::new(
            Selector::parse(".oke-w-reviewer-name").unwrap(),
            FieldType::Name,
        ),
    ];

    let reviews = Arc::new(tab).extract_reviews(&fields, None, None)?;

    let mut writer = Writer::from_path("reviews.csv")?;

    for review in reviews {
        writer.serialize(review)?;
    }

    writer.flush()?;

    Ok(())
}
