use chrono::{Duration, TimeZone, Utc};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::io::Write;

#[derive(Parser)]
struct AppArgs {
    #[clap(long = "data_file")]
    data_file: Option<String>,
    #[clap(short = 'd', long = "date")]
    date: Option<String>,
    #[clap(short = 'a', long = "amount")]
    amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct ExchangeInfo {
    date: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    usd_to_yen_opening_price: Option<f64>, // opening
    #[serde(deserialize_with = "csv::invalid_option")]
    usd_to_yen_closing_price: Option<f64>, // closing
    #[serde(deserialize_with = "csv::invalid_option")]
    eur_to_usd_opening_price: Option<f64>, // opening
    #[serde(deserialize_with = "csv::invalid_option")]
    eur_to_usd_closing_price: Option<f64>, // closing
}

fn create_utf8_csv_from(data_file_not_utf8: &String) {
    let s = fs::read(data_file_not_utf8).unwrap();
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
    let mut text = res.into_owned();

    text.retain(|c| c != '/');

    let text = text.replace("データコード", "date");
    let text = text.replace("FM08'FXERD01", "usd_to_yen_opening_price");
    let text = text.replace("FM08'FXERD04", "usd_to_yen_closing_price");
    let text = text.replace("FM08'FXERD31", "eur_to_usd_opening_price");
    let text = text.replace("FM08'FXERD34", "eur_to_usd_closing_price");

    let mut output_file = fs::File::create("output.csv").unwrap();
    output_file.write(text.as_bytes()).unwrap();
}

fn get_yesterday(date: &String) -> String {
    let vec_date = date.chars().collect::<Vec<_>>();
    let y: i32 = String::from_iter(vec_date[0..4].to_owned())
        .parse()
        .unwrap();
    let m: u32 = String::from_iter(vec_date[4..6].to_owned())
        .parse()
        .unwrap();
    let d: u32 = String::from_iter(vec_date[6..8].to_owned())
        .parse()
        .unwrap();

    let dt = Utc.ymd(y, m, d);
    let offset = Duration::days(1);
    let yesterday = (dt - offset).format("%Y%m%d").to_string();
    yesterday
}

fn get_target_exchange_info(date: &String) -> Option<ExchangeInfo> {
    let target_exchange_info = {
        let mut _target_exchange_info: Option<ExchangeInfo> = None;
        let mut reader = csv::Reader::from_path("output.csv").unwrap();
        for result in reader.deserialize() {
            let exchange_info: ExchangeInfo = result.unwrap();
            if &exchange_info.date == date {
                _target_exchange_info = Some(exchange_info);
                break;
            }
        }
        _target_exchange_info
    };

    match &target_exchange_info {
        None => {
            println!("Result: 入力された日付の為替データは存在しません。日付が正しいことを確認してください。日付が正しい場合は、データを確認してください。");
            None
        }
        Some(x) => match x.usd_to_yen_opening_price {
            None => {
                println!(
                    "{} は営業日ではないため前日のデータの取得を試みます。",
                    x.date
                );
                let yesterday = get_yesterday(&x.date);
                get_target_exchange_info(&yesterday)
            }
            _ => {
                println!("為替データは次の通りです。");
                println!(
                    "日付: {} || $→￥: 始値: {:?}, (終値): {:?} || €→$: 始値: {:?}, 終値: {:?}",
                    x.date,
                    x.usd_to_yen_opening_price.unwrap(),
                    x.usd_to_yen_closing_price.unwrap(),
                    x.eur_to_usd_opening_price.unwrap(),
                    x.eur_to_usd_closing_price.unwrap()
                );
                target_exchange_info
            }
        },
    }
}

fn main() {
    let args = AppArgs::parse();
    let mut target_exchange_info: Option<ExchangeInfo>;

    // --data_file
    if let Some(data_file) = args.data_file {
        println!("data_file: {:?}", data_file);
        create_utf8_csv_from(&data_file);
    }

    // --date
    if let Some(date) = args.date {
        target_exchange_info = get_target_exchange_info(&date);
    };

    // --amount
    if let Some(amount) = args.amount {
        println!("Amount: {}", amount);
    };
}
