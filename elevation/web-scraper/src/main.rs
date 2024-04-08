use futures::future::join_all;
use reqwest::Client;
use indicatif::ProgressBar;
use std::{fs::File, io::Write, path::Path, time::Duration};
use colored::Colorize;
use clap::{command, Arg, ArgAction};
use scraper::{Html, Selector};

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


async fn download(image: String, img_num: usize, path: String, client: Client) -> Result<(), Box<dyn std::error::Error>> {
    let response = async move {
        let future = client.get(image).send().await?;
        future.bytes().await
    }.await?;
    let name = format!("{}image{}.png", path, img_num); 
    let filename = Path::new(name.as_str())
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    let mut file = File::create(filename)?;
    file.write_all(&response)?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let matches = command!()
        .arg_required_else_help(true)
        .arg(
            Arg::new("recursive").short('r').action(ArgAction::SetTrue)
            .help("recursively downloads the images in all URLs received as parameters")
            .required(false)
        )
        .arg(
            Arg::new("length").short('l')
            .help("indicates the maximum depth level of the recursive download. If not indicated, it will be 5")
            .required(false)
        )
        .arg(
            Arg::new("to-path").short('p')
            .help("indicates the path where the downloaded files will be saved. If not specified, ./data/ will be used.")
            .required(false)
            )
        .arg(Arg::new("URLS").action(ArgAction::Append).num_args(1..).required(true))
        .get_matches();

    
    let mut query = Query::new();
    if matches.get_flag("recursive") {query.r_flag = true;}
    match matches.get_one::<String>("length") {
        Some(n) => query.l_flag = (*n).parse::<usize>()?,
        None    => query.l_flag = 5,
    };
    match matches.get_one::<String>("to-path") {
        Some(path)  => query.p_flag = String::from(path as &str),
        None        => query.p_flag = "./data/".to_string(),
    };
    match matches.get_many::<String>("URLS") {
        Some(urls)  => query.urls = urls.map(|v| v.as_str().to_string()).collect::<Vec<_>>(),
        None        => panic!("No URL provided"),
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
    let mut images: Vec<String> = Vec::new();
    for response in responses {
        match response {
            Ok(content) => {
                println!("{}", "Reading content...".green());
                let document = Html::parse_document(content.as_str());
                let selector = Selector::parse("img")?;
                for node in document.select(&selector) {
                    match node.attr("src") {
                        Some(img)   => images.push(img.to_string()),
                        None        => eprintln!("No images in node img"),
                    }
                }
            },
            Err(e)      => eprintln!("{} {}", "Error:".red(), e),
        };
    }
    if !query.r_flag {
        download(String::from(images[0].as_str()), 0, query.p_flag, client.clone()).await?;
    } else {
        let mut i: usize = 0;
        for img in images {
            if i == query.l_flag {
                break;
            }
            download(String::from(img.as_str()), i, String::from(query.p_flag.as_str()), client.clone()).await?;
            println!("{} {}", "Downloaded:".blue(), img);
            i += 1;
        }
    }

    spinner.finish();
    
    Ok(())
}
