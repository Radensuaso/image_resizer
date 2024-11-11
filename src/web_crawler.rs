use reqwest;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use tokio::time::{sleep, Duration};
use url::Url;

pub async fn crawl(start_url: &str, max_depth: usize) -> Result<(), Box<dyn Error>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_url.to_string(), 0));
    visited.insert(start_url.to_string());

    while let Some((url, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }

        println!("Crawling {} (depth {})", url, depth);

        let response = reqwest::get(&url).await?.text().await?;
        let links = parse_links(&response, &url)?;

        for link in links {
            if !visited.contains(&link) {
                visited.insert(link.clone());
                queue.push_back((link, depth + 1));
            }
        }

        // Sleep for 500 milliseconds between requests
        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

fn parse_links(html: &str, base_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let base = Url::parse(base_url)?;
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(link) = base.join(href) {
                links.push(link.to_string());
            }
        }
    }

    Ok(links)
}
