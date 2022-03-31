use std::env;
use structopt::StructOpt;
use colored::*;
use scraper::{Html, Selector};
use reqwest::StatusCode;

mod utils;

#[derive(StructOpt, Debug)]
#[structopt(name = "arguments")]
struct Arguments {
    #[structopt(name = "ARGUMENTS")]
    rest: Vec<String>
}

impl Arguments { 
    fn get_joined(&self, _seperator: &str) -> String {
        self.rest.join("+")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let arguments = Arguments::from_args();
        let url = String::from("https://www.urbandictionary.com/define.php?term=") + &arguments.get_joined(" ");
        
        // prints out stuff in Table

        //println!("{} {}", "URL:".bold(), url.normal());
        meaning(&url, &arguments.get_joined(" "));
        println!("{} {}", "URL:".bold(), url.normal());
    } else {
        println!("{}{}", "ERROR".red().bold(), ": Please enter a valid query".normal());
    }
}

// gets the meaning and example
#[tokio::main]
async fn meaning(url: &str, title: &str) {
    let client = utils::get_client();
    let result = client.get(url).send().await.unwrap();
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    let document = Html::parse_document(&raw_html);
    let example_selector = Selector::parse("div.italic").unwrap();
    let article_selector = Selector::parse("div.meaning").unwrap();

    for element in document.select(&article_selector) {

        let inner = element.inner_html().to_string();
        let result = remove_tags_from(&inner).replace(".", ". ").replace("  ", " ");


        let title_print = title.replace("+", " ");
        println!("{}{}: \n\"{}\"\n", "Meaning Of ".bold(), title_print.bold(), result.normal());

        for element2 in document.select(&example_selector) {
            let inner2 = element2.inner_html().to_string();
            let result2 = remove_tags_from(&inner2).replace(".",". ").replace("  ", " ");
            println!("{}\n{}\n", "Example:".bold(), result2.normal());
            break;
        }

        break;
    }
}

// removes html tags
fn remove_tags_from(string: &str) -> String {
    let mut depth: u64 = 0;

    string.chars().filter(|c| {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            _ => return depth == 0,
        }

        false
    }).collect::<String>()    
}
