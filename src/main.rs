use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::time::{sleep, Duration};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    // mojang_test(String::from("27110498ea3b4f3b8467ab0a185e415f")).await;
    hypixel_test().await
}

async fn hypixel_test() {
    let url = format!("https://api.hypixel.net/skyblock/auctions");

    let request = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap();

    match request.status() {

        reqwest::StatusCode::OK => {},
        _ => {
            println!("It did not worked");
            return;
        }

    }

    let x: AuctionRoot = request.json().await.unwrap();
    let mut t = vec!(x.clone());
    let auctions = x.total_auctions.clone();
    let pages = x.total_pages;
    println!("Total pages: {}", pages);

    for f in 1..pages{
        println!("Page: {}", f);
        let url = format!("https://api.hypixel.net/skyblock/auctions?page={}", f);

        let request = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .unwrap();

        match request.status() {

            reqwest::StatusCode::OK => {},
            _ => {
                println!("It did not worked");
                println!("{}", request.status().to_string());
                return;
            }

        }

        let x: AuctionRoot = request.json().await.unwrap();

        t.push(x)
    }
    println!("Auctions: {}", auctions);
    let mut count = 1;
    let start = SystemTime::now();

    for l in t {
        for i in l.auctions {
            for j in i.coop {
                let now = SystemTime::now();
                let since_the_epoch = now
                    .duration_since(start)
                    .expect("Time went backwards");
                sleep(Duration::from_secs_f32(1.05)).await;
                mojang_test(j).await;
                println!("{} Count: {}", since_the_epoch.as_secs(), count);
                println!("{} Auctions left: {}", since_the_epoch.as_secs(),auctions - count);
                count+=1;
            }
        }
    }

    println!("done")
}

async fn mojang_test(uuid: String) {
    let url = format!("https://api.mojang.com/user/profiles/{uuid}/names", uuid=uuid);

    let request = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .unwrap();

    match request.status() {

        reqwest::StatusCode::OK => {
        },
        _ => {
            println!("It did not worked");
            return;
        }

    }

    let x:Vec<Names> = request.json().await.unwrap();
    let x = x.last().clone();
    let x = match x {
        Some(t) => t.to_owned().name,
        None => return
    };
    // println!("Name {}", x);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Names {
    name: String,
    #[serde(rename = "changedToAt")]
    changed_to_at: Option<u64>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionRoot {
    pub success: bool,
    pub page: i64,
    pub total_pages: i64,
    pub total_auctions: i64,
    pub last_updated: i64,
    pub auctions: Vec<Auction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auction {
    pub uuid: String,
    pub auctioneer: String,
    #[serde(rename = "profile_id")]
    pub profile_id: String,
    pub coop: Vec<String>,
    pub start: i64,
    pub end: i64,
    #[serde(rename = "item_name")]
    pub item_name: String,
    #[serde(rename = "item_lore")]
    pub item_lore: String,
    pub extra: String,
    pub category: String,
    pub tier: String,
    #[serde(rename = "starting_bid")]
    pub starting_bid: i64,
    #[serde(rename = "item_bytes")]
    pub item_bytes: String,
    pub claimed: bool,
    #[serde(rename = "claimed_bidders")]
    pub claimed_bidders: Vec<Value>,
    #[serde(rename = "highest_bid_amount")]
    pub highest_bid_amount: i64,
    #[serde(rename = "last_updated")]
    pub last_updated: i64,
    pub bin: bool,
    pub bids: Vec<Bid>,
    #[serde(rename = "item_uuid")]
    pub item_uuid: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(rename = "auction_id")]
    pub auction_id: String,
    pub bidder: String,
    #[serde(rename = "profile_id")]
    pub profile_id: String,
    pub amount: i64,
    pub timestamp: i64,
}
