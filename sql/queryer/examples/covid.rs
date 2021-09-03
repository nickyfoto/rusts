use anyhow::Result;
// use polars::prelude::*;
// use std::io::Cursor;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let data = reqwest::get(url).await?.text().await?;

    // let df = CsvReader::new(Cursor::new(data))
    //     .infer_schema(Some(16))
    //     .finish()?;

    // let filtered = df.filter(&df["new_deaths"].gt(500))?;
    // println!(
    //     "{:?}",
    //     filtered.select((
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ))
    // );

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} WHERE new_deaths >= 500 ORDER BY new_cases DESC",
        url);
    let df1 = query(sql).await?;
    println!("{:?}", df1);
    Ok(())
}