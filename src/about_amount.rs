use crate::common::ExchangeInfo;
use crate::common::OpenOrClose;
use crate::my_utils;
use ansi_term::Colour;

pub fn print_calculation_results(
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
            println!("✨ {} ✨", Colour::Cyan.paint("始値で換算した結果です"));
            println!("---------------------------");
        }
        OpenOrClose::ClosingPrice => {
            usd_to_yen_rate = exchange_info.usd_to_yen_closing_price.unwrap();
            eur_to_usd_rate = exchange_info.eur_to_usd_closing_price.unwrap();
            println!("---------------------------");
            // println!("✨ 終値で換算した結果です ✨");
            println!("{}", Colour::Cyan.paint("✨ 終値で換算した結果です ✨"));
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
