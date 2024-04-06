use reqwest::Url;
use select::{
    document::Document,
    predicate::Name,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No URL provided, using default example.");
        "https://hyper.rs".into()
    };

    eprint!("Fetching {url:?}...");

    let res = reqwest::get(url).await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{body}");

    Ok(())
}
