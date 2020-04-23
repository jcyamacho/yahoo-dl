use chrono::{Date, Local, TimeZone};
use chrono_tz::America::New_York;
use reqwest::Client;

use uuid::Uuid;

pub use chrono;
pub use chrono_tz;

const YAHOO_URL: &'static str = "https://finance.yahoo.com";
const YAHOO_QUERY_URL: &'static str = "https://query1.finance.yahoo.com";
const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36";

fn random_str() -> String {
    let uuid = Uuid::new_v4().to_string();
    let mut segments = uuid.split("-");
    let seg = segments.next().unwrap();
    String::from(seg)
}

fn to_ny_unix<Tz: TimeZone>(date: Option<Date<Tz>>, default: i64) -> i64 {
    match date {
        Some(dt) => {
            let ny_time = dt.with_timezone(&New_York);
            ny_time.and_hms(0, 0, 0).timestamp()
        }
        None => default,
    }
}

pub struct Yahoo {
    client: Client,
    crumb: Option<String>,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
}

impl Period {
    fn to_str(self) -> &'static str {
        use Period::*;
        match self {
            Daily => "d",
            Weekly => "wk",
            Monthly => "mo",
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Event {
    History,
    Dividend,
    Split,
}

impl Event {
    fn to_str(self) -> &'static str {
        use Event::*;
        match self {
            History => "history",
            Dividend => "div",
            Split => "split",
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Yahoo {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .gzip(true)
            .build()
            .unwrap();
        Self {
            client: client,
            crumb: None,
        }
    }
    async fn cookies(&self) -> Result<()> {
        let rnd = random_str();
        let url = format!("{}?{}", YAHOO_URL, rnd);
        self.client.get(&url).send().await?;
        Ok(())
    }
    async fn crumb(&self) -> Result<String> {
        let url = format!("{}/v1/test/getcrumb", YAHOO_QUERY_URL);
        let res = self.client.get(&url).send().await?.text().await?;
        Ok(res)
    }
    async fn init(&mut self) -> Result<()> {
        if self.crumb.is_none() {
            self.cookies().await?;
            let crumb = self.crumb().await?;
            self.crumb = Some(crumb);
        }
        Ok(())
    }
    pub async fn download<Tz: TimeZone>(
        &mut self,
        symbol: &str,
        start_date: Option<Date<Tz>>,
        end_date: Option<Date<Tz>>,
        period: Period,
        event: Event,
    ) -> Result<String> {
        self.init().await?;

        let crumb = self.crumb.as_ref().unwrap();
        let start = to_ny_unix(start_date, 0);
        let end = to_ny_unix(end_date, Local::now().timestamp());

        let mut url = format!("{}/v7/finance/download/{}?", YAHOO_QUERY_URL, symbol);
        url.push_str(&format!("period1={}&", start));
        url.push_str(&format!("period2={}&", end));
        url.push_str(&format!("interval=1{}&", period.to_str()));
        url.push_str(&format!("events={}&", event.to_str()));
        url.push_str(&format!("crumb={}", crumb));

        let text = self.client.get(&url).send().await?.text().await?;

        Ok(text)
    }
}
