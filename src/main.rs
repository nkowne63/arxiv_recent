use anyhow::Result;
use arxiv::ArxivQueryBuilder;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    let dtd = Utc::now().date_naive();
    let dt = dtd.and_hms_opt(0, 0, 0);
    let ago = dt.unwrap() - chrono::Duration::days(7);
    println!("start datetime: {}", ago);
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
            dt > ago
        })
        .collect::<Vec<_>>();
    println!("count: {}", arxivs.len());
    for arxiv in arxivs {
        println!("{} {}", arxiv.id, arxiv.title.replace('\n', ""),);
    }
    Ok(())
}
