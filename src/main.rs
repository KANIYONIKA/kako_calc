use ansi_term::Colour;
use clap::Parser;
use kako_culc::my_utils;
use serde::Deserialize;
use std::fs;
use std::io::Write;

#[derive(Parser)]
struct AppArgs {
    #[clap(short = 'd', long = "date")]
    date: Option<String>,
    #[clap(short = 'a', long = "amount")]
    amount: Option<f64>,
    #[clap(short = 'f', long = "data_file")]
    data_file: Option<String>,
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
            println!("{}", Colour::Purple.paint("入力された日付の為替データは存在しません。日付が正しいことを確認してください。日付が正しい場合は、データを確認してください。"));
            None
        }
        Some(x) => match x.usd_to_yen_opening_price {
            None => {
                println!(
                    "{} は営業日ではないため前日のデータの取得を試みます。",
                    x.date
                );
                let yesterday = my_utils::get_yesterday(&x.date);
                get_target_exchange_info(&yesterday)
            }
            _ => {
                println!("------------------------------------------");
                println!(
                    "{}",
                    Colour::Blue.paint("💰計算に利用する為替データは次の通りです💰")
                );
                println!("------------------------------------------");

                println!(
                    "  日付: {} || $ → ¥: 始値: {:?}, 終値: {:?} || € → $: 始値: {:?}, 終値: {:?}",
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

enum OpenOrClose {
    OpeningPrice,
    ClosingPrice,
}

fn print_calculation_results(
    open_or_close: OpenOrClose,
    amount: f64,
    exchange_info: &ExchangeInfo,
) {
    let usd_to_yen_rate: f64;
    let eur_to_usd_rate: f64;

    match open_or_close {
        OpenOrClose::OpeningPrice => {
            usd_to_yen_rate = exchange_info.usd_to_yen_opening_price.unwrap();
            eur_to_usd_rate = exchange_info.eur_to_usd_opening_price.unwrap();
            println!("---------------------------");
            // println!("✨ 始値で換算した結果です ✨");
            println!("{}", Colour::Blue.paint("✨ 始値で換算した結果です ✨"));
            println!("---------------------------");
        }
        OpenOrClose::ClosingPrice => {
            usd_to_yen_rate = exchange_info.usd_to_yen_closing_price.unwrap();
            eur_to_usd_rate = exchange_info.eur_to_usd_closing_price.unwrap();
            println!("---------------------------");
            // println!("✨ 終値で換算した結果です ✨");
            println!("{}", Colour::Blue.paint("✨ 終値で換算した結果です ✨"));
            println!("---------------------------");
        }
    }

    let usd_to_yen = amount * usd_to_yen_rate;
    println!(
        "  $ {} => ¥ {}",
        amount,
        my_utils::round_decimal_pt(usd_to_yen, 4),
    );

    let usd_to_eur = amount / eur_to_usd_rate;
    println!(
        "  $ {} => € {}",
        amount,
        my_utils::round_decimal_pt(usd_to_eur, 4),
    );

    let yen_to_usd = amount / usd_to_yen_rate;
    println!(
        "  ¥ {} => $ {}",
        amount,
        my_utils::round_decimal_pt(yen_to_usd, 4),
    );

    let yen_to_eur = (amount / usd_to_yen_rate) / (eur_to_usd_rate);
    println!(
        "  ¥ {} => € {}",
        amount,
        my_utils::round_decimal_pt(yen_to_eur, 4),
    );

    let eur_to_usd = amount * eur_to_usd_rate;
    println!(
        "  € {} => $ {}",
        amount,
        my_utils::round_decimal_pt(eur_to_usd, 4),
    );

    let eur_to_yen = (amount * eur_to_usd_rate) * (usd_to_yen_rate);
    println!(
        "  € {} => ¥ {}",
        amount,
        my_utils::round_decimal_pt(eur_to_yen, 4),
    );
}

fn main() {
    let args = AppArgs::parse();
    let mut target_exchange_info: Option<ExchangeInfo> = None;

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
        match &target_exchange_info {
            None => {
                println!(
                    "{}（Amount: {}）",
                    Colour::Purple.paint("日付を指定してください。"),
                    amount
                );
            }
            Some(x) => {
                print_calculation_results(OpenOrClose::OpeningPrice, amount, x);
                print_calculation_results(OpenOrClose::ClosingPrice, amount, x)
            }
        }
    };
}
