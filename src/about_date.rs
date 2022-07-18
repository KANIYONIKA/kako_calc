use crate::common::{ExchangeInfo, EXCHANGE_INFO_FILE_NAME};
use crate::my_utils;
use ansi_term::Colour;

pub fn get_target_exchange_info(date: &String) -> Option<ExchangeInfo> {
    let target_exchange_info = {
        let mut _target_exchange_info: Option<ExchangeInfo> = None;
        let mut reader = csv::Reader::from_path(EXCHANGE_INFO_FILE_NAME).unwrap();

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
            println!("{}", Colour::Purple.paint("入力された日付の為替データは存在しません。日付が正しいことを確認してください。\n日付が正しい場合は「setup」を実施してください。"));
            None
        }
        Some(x) => match x.usd_to_yen_opening_price {
            None => {
                println!(
                    "{} データなし｜非営業の可能性があるため前日のデータ取得を試みます｜{} の確認をお勧めします。",
                    x.date,
                    EXCHANGE_INFO_FILE_NAME,

                );
                let yesterday = my_utils::get_yesterday(&x.date);
                get_target_exchange_info(&yesterday)
            }
            _ => {
                println!("------------------------------------------");
                println!(
                    "{}",
                    Colour::Yellow.paint("💰計算に利用する為替データは次の通りです💰")
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
