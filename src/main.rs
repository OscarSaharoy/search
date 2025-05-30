use reqwest::Client;
use scraper::{Html, Selector};
use textwrap::fill;
use std::env;
use colored::*;
use urlencoding::encode;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a search query as an argument.");
        std::process::exit(1);
    }
    let query = itertools::Itertools::intersperse(args.into_iter().skip(1), " ".to_string()).collect::<String>();

    let ecosia_url = "https://www.ecosia.org/search";
    let full_ecosia_url = format!("{}?q={}", ecosia_url, encode(&query));
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let html_content = client
        .get(&full_ecosia_url)
        .header("Upgrade-Insecure-Requests", "1")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-User", "?1")
        .header("Sec-Fetch-Dest", "document")
        .header("Accept-Language", "en-GB,en-US;q=0.9,en;q=0.8")
        .header("Priority", "u=0, i")
        .header("Accept-Encoding", "gzip, deflate, br")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html_content);
    let result_selector = Selector::parse(".web-result").unwrap();
    let result_link_selector = Selector::parse(".result__title .result__link").unwrap();
    let result_description_selector = Selector::parse(".result__description").unwrap();

    println!();

    let results = document.select(&result_selector).collect::<Vec<_>>();
    if results.len() == 0 {
        panic!("No results found! There is probably an issue or change made to ecosia so that this tool no longer works, sorry :(");
    }

    for element in results {

        if let Some(result_link_el) = element.select(&result_link_selector).next() {
            let href = result_link_el.value().attr("href").unwrap_or("#");
            let text = result_link_el.text().collect::<String>();
            println!("{} [{}]", text.trim().cyan(), href.magenta());
        }

        if let Some(result_description_el) = element.select(&result_description_selector).next() {
            let result_text =  result_description_el.text().collect::<String>();
            println!("{}\n", fill(result_text.trim(), 80));
        }
    }

    println!("{} {}", String::from("Open in ecosia:").green(), full_ecosia_url.green());

    Ok(())
}

