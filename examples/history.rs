use yahoo_dl::{chrono::Local, Event, Period, Result, Yahoo};

#[tokio::main]
async fn main() -> Result<()> {
    let mut dwl = Yahoo::new();
    let res = dwl
        .download::<Local>("GOOG", None, None, Period::Monthly, Event::History)
        .await?;
    println!("{}", res);
    Ok(())
}
