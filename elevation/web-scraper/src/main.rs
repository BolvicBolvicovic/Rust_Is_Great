use futures::future::join_all;
use reqwest::Client;
use indicatif::ProgressBar;
use std::{error::Error, path, time::Duration};
use colored::Colorize;
use clap::{arg, command, Arg, ArgAction, Error};


struct Query {
    r_flag: bool,
    l_flag: usize,
    p_flag: String,
    urls  : Vec<String>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            r_flag: false,
            l_flag: 1,
            p_flag: String::from(""),
            urls  : Vec::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let matches = command!()
        .arg(
            arg!(
                -r "recursively downloads the images in all URLs received as parameters"
            )
            .required(false)
        )
        .arg(
            arg!(
                -l <N> "indicates the maximum depth level of the recursive download. If not indicated, it will be 5"
            )
            .required(false)
        )
        .arg(
            arg!(
                -p <PATH> "indicates the path where the downloaded files will be saved. If not specified, ./data/ will be used."
            )
            .required(false)
        )
        .arg(Arg::new("URLS").action(ArgAction::Append).num_args(1..))
        .get_matches();

    
    let mut query = Query::new();
    if matches.get_flag("r") {query.r_flag = true;}
    if matches.get_flag("l") {
        match matches.get_one("N") {
            Some(n) => query.l_flag = *n,
            None    => query.l_flag = 5,
        };
    }
    if matches.get_flag("p") {
        match matches.get_one("PATH") {
            Some(path)  => query.p_flag = String::from(path as &str),
            None        => query.p_flag = "./data/".to_string(),
        }
    }
    match matches.get_many::<String>("URLS") {
        Some(urls)  => query.urls = urls.map(|v| v.as_str().to_string()).collect::<Vec<_>>(),
        None        => eprintln!("No URL provided"),
    };


    let client = Client::new();
    let mut futures = Vec::new();
    let spinner = ProgressBar::new_spinner();
    eprintln!("{}", "Fetching urls...".green());
    spinner.enable_steady_tick(Duration::from_millis(100));

    for url in query.urls {
       let client = client.clone();
       let future = async move {
            let response = client.get(url).send().await?;
            response.text().await
       };
       futures.push(future);
    }

    let responses = join_all(futures).await;
    
    spinner.finish();

    eprintln!("{}", "Parsing responses...".green());
    spinner.enable_steady_tick(Duration::from_millis(100));
    for response in responses {
        match response {
            Ok(content) => println!("{} {}", "Fetched content:".green(), content),
            Err(e)      => eprintln!("{} {}", "Error:".red(), e),
        }
    }

    spinner.finish();
    
    Ok(())
}
