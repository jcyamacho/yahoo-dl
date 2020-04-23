use yahoo_dl::{chrono::Local, Event, Period, Result, Yahoo};

#[tokio::main]
async fn main() -> Result<()> {
    let mut dwl = Yahoo::new();
    let res = dwl
        .download::<Local>("MSFT", None, None, Period::Daily, Event::Dividend)
        .await?;
    println!("{}", res);
    Ok(())
}
