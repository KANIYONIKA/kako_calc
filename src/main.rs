use ansi_term::Colour;
use clap::Parser;
use kako_calc::about_amount;
use kako_calc::about_date;
use kako_calc::about_file_path;
use kako_calc::common::AppArgs;
use kako_calc::common::ExchangeInfo;
use kako_calc::common::OpenOrClose;
use kako_calc::my_utils;
use std::cmp::Ordering;
use std::io;

fn main() {
    my_utils::executed_shell_command("clear".to_string());

    let args = AppArgs::parse();
    let mut target_exchange_info: Option<ExchangeInfo> = None;

    // --data_file (do first)
    if let Some(data_file) = &args.file_path {
        println!("data_file: {:?}", data_file);
        about_file_path::create_utf8_csv_from(&data_file);
    }

    // --date
    if let Some(date) = &args.date {
        target_exchange_info = about_date::get_target_exchange_info(&date);
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
                about_amount::print_calculation_results(OpenOrClose::OpeningPrice, amount, x);
                about_amount::print_calculation_results(OpenOrClose::ClosingPrice, amount, x);
                println!("");
            }
        }
    };

    // no args => menu
    if args.is_all_none() {
        loop {
            println!(
                "{}",
                Colour::Yellow.paint("✨✨✨💰 Welcome to kako_calc 💰✨✨✨")
            );
            println!("{}", Colour::Cyan.paint("為替データセットアップ未完了 =>「setup」と入力\nセットアップ済み =>「yyyymmdd amount」（日付と金額）を入力"));

            let mut input_command = String::new();
            io::stdin()
                .read_line(&mut input_command)
                .expect("Failed to read line");
            let input_command = input_command.trim().to_string();

            match input_command.cmp(&String::from("setup")) {
                // setup
                Ordering::Equal => {
                    println!("{}", Colour::Cyan.paint("日本銀行「時系列統計データ 検索サイト」から、「外国為替相場状況（日次）」をダウンロードしてください。\n※URL※ https://www.stat-search.boj.or.jp/ssi/cgi-bin/famecgi2?cgi=$nme_a000&lstSelection=FM08\n必要な項目は「ドル・円」と「ユーロ・ドル」の「9時時点」と「17時点」です。（全項目可）\nダウンロードが完了したらファイルのパスを入力するかドラッグ&ドロップしてEnterを押してください。"));
                    let mut input_command = String::new();
                    io::stdin()
                        .read_line(&mut input_command)
                        .expect("Failed to read line");
                    let input_command = input_command.trim().to_string();
                    about_file_path::create_utf8_csv_from(&input_command);
                }
                // calculate
                _ => {
                    let vec_date = input_command.chars().collect::<Vec<_>>();
                    let yyyymmdd = String::from_iter(vec_date[0..8].to_owned());
                    let amount: f64 = String::from_iter(vec_date[8..].to_owned())
                        .trim()
                        .parse()
                        .unwrap();
                    target_exchange_info = about_date::get_target_exchange_info(&yyyymmdd);
                    if let Some(x) = &target_exchange_info {
                        about_amount::print_calculation_results(
                            OpenOrClose::OpeningPrice,
                            amount,
                            x,
                        );
                        about_amount::print_calculation_results(
                            OpenOrClose::ClosingPrice,
                            amount,
                            x,
                        );
                        println!("");
                    }
                }
            }
        }
    }
}
