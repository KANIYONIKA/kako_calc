use clap::Parser;
use serde::Deserialize;

pub const EXCHANGE_INFO_FILE_NAME: &str = "exchange_info_file.csv";

#[derive(Parser, Debug)]
pub struct AppArgs {
    #[clap(short = 'd', long = "date")]
    pub date: Option<String>,
    #[clap(short = 'a', long = "amount")]
    pub amount: Option<f64>,
    #[clap(short = 's', long = "setup")]
    pub file_path: Option<String>,
}
impl AppArgs {
    pub fn is_all_none(&self) -> bool {
        if self.date == None && self.amount == None && self.file_path == None {
            return true;
        }
        false
    }
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfo {
    pub date: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub usd_to_yen_opening_price: Option<f64>, // opening
    #[serde(deserialize_with = "csv::invalid_option")]
    pub usd_to_yen_closing_price: Option<f64>, // closing
    #[serde(deserialize_with = "csv::invalid_option")]
    pub eur_to_usd_opening_price: Option<f64>, // opening
    #[serde(deserialize_with = "csv::invalid_option")]
    pub eur_to_usd_closing_price: Option<f64>, // closing
}
pub enum OpenOrClose {
    OpeningPrice,
    ClosingPrice,
}
#[derive(Debug)]
pub struct DirectoryInfo {
    pub path_executed: String,
    pub exchange_info_file_path: String,
}
