use core::panic;

use anyhow::Result;
use arxiv::ArxivQueryBuilder;
use chrono::{DateTime, NaiveDate, Utc};
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    start_day: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    let cli = Cli::parse();
    let start_day = cli
        .start_day
        .map(|s| {
            NaiveDate::parse_from_str(&s, "%y-%m-%d").map(|rn| rn.and_hms_opt(0, 0, 0).unwrap())
        })
        .unwrap()
        .unwrap_or({
            let dtd = Utc::now().date_naive();
            let dt = dtd.and_hms_opt(0, 0, 0);
            dt.unwrap() - chrono::Duration::days(7)
        });
    println!("start datetime: {}", start_day);
    let query = ArxivQueryBuilder::new()
        .search_query("cat:cs.PL cat:quant-ph")
        .start(0)
        .max_results(2000)
        .sort_by("lastUpdatedDate")
        .sort_order("descending")
        .build();
    let arxivs = arxiv::fetch_arxivs(query)
        .await?
        .into_iter()
        .filter(|arxiv| {
            let dt = DateTime::parse_from_rfc3339(&arxiv.updated);
            let dt = dt.unwrap().date_naive().and_hms_opt(0, 0, 0).unwrap();
            dt > start_day
        })
        .collect::<Vec<_>>();
    println!("count: {}", arxivs.len());
    for arxiv in arxivs {
        // remove not "...v1"
        if !arxiv.id.ends_with("v1") {
            continue;
        }
        println!("{} {}", arxiv.id, arxiv.title.replace('\n', ""),);
    }
    Ok(())
}
