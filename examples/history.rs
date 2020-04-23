use yahoo_dl::{chrono::Local, Event, Period, Result, Yahoo};

#[tokio::main]
async fn main() -> Result<()> {
    let mut yahoo = Yahoo::new();
    let res = yahoo
        .download::<Local>("GOOG", None, None, Period::Monthly, Event::History)
        .await?;
    println!("{}", res);
    Ok(())
}
