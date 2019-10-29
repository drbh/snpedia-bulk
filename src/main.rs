extern crate mediawiki;
use serde_json::json;
use std::collections::HashMap;

mod page_model;
use crate::page_model::ResponseSNPedia;

// to limit scrape speed
use std::thread;
use std::time::Duration;

fn fetch_single_page(search: &str) -> String {
    let api = mediawiki::api::Api::new("http://bots.snpedia.com/api.php").unwrap();
    // Query parameters
    let params: HashMap<_, _> = vec![
        ("action".to_string(), "parse".to_string()),
        ("page".to_string(), search.to_string()),
    ]
    .into_iter()
    .collect();

    // Run query; this will automatically continue if more results are available, and merge all results into one
    let res = api.get_query_api_json_all(&params).unwrap();
    let data = res.as_object().unwrap();
    let str_data = json!(data).to_string();
    let p: ResponseSNPedia = serde_json::from_str(&str_data).unwrap();
    p.parse.text.field
}

fn fetch_all_snps() -> String {
    let api = mediawiki::api::Api::new("http://bots.snpedia.com/api.php").unwrap();

    // Query parameters
    let params: HashMap<_, _> = vec![
        ("action".to_string(), "query".to_string()),
        ("cmtitle".to_string(), "Category:Is_a_snp".to_string()),
        ("cmlimit".to_string(), "500".to_string()),
        ("list".to_string(), "categorymembers".to_string()),
        // ("format".to_string(), "json".to_string()),
        // ("prop".to_string(), "categoryinfo".to_string()),

        // "action": "query",
        // "cmtitle": "Category:Physics",
        // "cmlimit": "20",
        // "list": "categorymembers",
        // "format": "json"

        // ("titles".to_string(), "Rs53576".to_string()),
        // ("list".to_string(), "categorymembers".to_string()),
        // ("cmtitle".to_string(), "Category:Is_a_snp".to_string()),
        // ("cllimit".to_string(), "100".to_string()),
    ]
    .into_iter()
    .collect();

    let res = api.get_query_api_json_all(&params).unwrap();
    let data = res.as_object().unwrap();
    let str_data = json!(data).to_string();
    str_data
}

fn scrape_page(html_page: String) -> Vec<Vec<String>> {
    let fragment = Html::parse_document(&html_page);
    let title_selector = Selector::parse("table.smwtable > tbody > tr > td").unwrap();
    let b = fragment.select(&title_selector);

    // use scraper::Selector;
    // let selector = Selector::parse("h1.foo").unwrap();

    let mut count = 0;
    let mut row_count = 0;
    let mut rows: Vec<Vec<String>> = vec![];
    for it in b {
        count += 1;
        let h = it.text().collect::<Vec<_>>();

        let k = h
            .iter()
            .map(|n| n.trim().to_string())
            .collect::<Vec<String>>()
            .clone();

        let value = k[0].to_string();

        // println!("{} {} {}", (count - 1) % 3, row_count, value);

        if rows.len() == row_count {
            rows.push(vec![String::new(); 3])
        }
        rows[row_count][(count - 1) % 3] = value;
        if count % 3 == 0 {
            row_count += 1;
        };
    }
    // println!("{:#?}", rows);
    rows
}

fn download_parse_many() {
    let pages = vec![
        "Rs53576",
        "Rs7495174",
        "Rs1815739",
        "Rs6152",
        "Rs1234",
        "Rs333",
    ];

    for page in pages {
        let html_page = fetch_single_page(page);
        let values = scrape_page(html_page);
        println!("{:#?}", values);
        thread::sleep(Duration::from_millis(5000))
    }
}

use scraper::{Html, Selector};

fn _get_json_values() {
    let list_of_snp_identifiers = fetch_all_snps();
    println!("{}", list_of_snp_identifiers);
}

use std::fs::File;
use std::path::Path;

use std::io::prelude::*;

fn fetch_and_store() {
    // let list_of_snp_identifiers = fetch_all_snps();
    // println!("{}", list_of_snp_identifiers);

    let json_file_path = Path::new("index.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized_camera: Root =
        serde_json::from_reader(json_file).expect("error while reading json");

    println!("{}", "File Loaded");

    let members = deserialized_camera.query.categorymembers;
    let mut revd = members.iter().rev();

    let nt = 9274;

    revd.nth(nt.clone());
    // let mut all_results: Vec<Vec<Vec<String>>> = vec![];
    let mut count = nt.clone();
    for res in revd {
        count += 1;

        println!("{} {}", count, res.title);

        let html_page = fetch_single_page(&res.title);
        let values = scrape_page(html_page);
        // println!("{:#?}", values);
        // all_results.push(values);

        let mut file = File::create(format!("data/{}.json", &res.title)).unwrap();
        file.write_all(json!(values).to_string().as_bytes())
            .unwrap();

        thread::sleep(Duration::from_millis(125))
    }
    // let str_data = json!(all_results).to_string();
    // println!("{}", str_data);
}

extern crate walkdir;
use walkdir::WalkDir;

use std::{env, fs};

use std::error::Error;

fn tester() -> Result<(), Box<dyn Error>> {
    let mut big_data_map = HashMap::new();
    // let mut results: Vec<String> = Vector::new();

    for entry in WalkDir::new("./data/")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;
        if f_name.ends_with(".json") {
            let filename = format!("./data/{fname}", fname = f_name);
            let xyz = format!("{fname}", fname = f_name);
            let mut content = String::new();
            let mut f = File::open(filename.clone())?;
            f.read_to_string(&mut content).unwrap();
            let v: Vec<Vec<String>> = serde_json::from_str(&content).unwrap();
            // println!("{} {:?}", filename, v.len());
            if v.len() > 0 {
                //
                // println!("{:?}", v);
                for c in v {
                    let genes = c[0].clone();
                    let mag = c[0].clone();
                    let text = c[2].clone();

                    let just_snp = xyz.clone();
                    let _snp = just_snp.trim_end_matches(".json");

                    let gene_p = genes.trim_end_matches(")").trim_start_matches("(");

                    let key = format!("{} {}", _snp, gene_p);
                    big_data_map.insert(key, text);

                    // let (genes, mag, text) = c;
                    //
                    // big_data_map.insert(key, text);
                    //     // results.push()
                    //     // unimplemented!();
                }
            }
        }
    }

    let mut file = File::create("./complete_data.json").unwrap();
    file.write_all(json!(big_data_map).to_string().as_bytes())
        .unwrap();

    Ok(())
}

fn main() {
    tester();
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub batchcomplete: String,
    pub query: Query,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub categorymembers: Vec<Categorymember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Categorymember {
    pub ns: i64,
    pub pageid: i64,
    pub title: String,
}
