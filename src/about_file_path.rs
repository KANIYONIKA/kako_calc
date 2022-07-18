use crate::common::EXCHANGE_INFO_FILE_NAME;
use ansi_term::Colour;
use std::fs;
use std::io::Write;

pub fn create_utf8_csv_from(data_file_not_utf8: &String) {
    let mut data_file_not_utf8 = String::from(data_file_not_utf8).trim().to_string();
    data_file_not_utf8.retain(|c| c != '"');
    data_file_not_utf8.retain(|c| c != '\'');

    let file_contents = fs::read(data_file_not_utf8).unwrap();
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&file_contents);
    let mut text = res.into_owned();

    text.retain(|c| c != '/');

    let text = text.replace("データコード", "date");
    let text = text.replace("FM08'FXERD01", "usd_to_yen_opening_price");
    let text = text.replace("FM08'FXERD04", "usd_to_yen_closing_price");
    let text = text.replace("FM08'FXERD31", "eur_to_usd_opening_price");
    let text = text.replace("FM08'FXERD34", "eur_to_usd_closing_price");

    let mut exchange_info_file = fs::File::create(EXCHANGE_INFO_FILE_NAME).unwrap();
    exchange_info_file.write(text.as_bytes()).unwrap();

    println!(
        "{} - {}",
        Colour::Yellow.paint("為替データのセットアップが完了しました。"),
        Colour::Yellow.paint(EXCHANGE_INFO_FILE_NAME),
    );
}
