use yahoo_dl::{chrono::Local, Event, Period, Result, Yahoo};

#[tokio::main]
async fn main() -> Result<()> {
    let mut yahoo = Yahoo::new();
    let res = yahoo
        .download::<Local>("MSFT", None, None, Period::Daily, Event::Dividend)
        .await?;
    println!("{}", res);
    Ok(())
}
