use crate::common::{DirectoryInfo, EXCHANGE_INFO_FILE_NAME};
use chrono::{Duration, TimeZone, Utc};
use std::env;

pub fn get_yesterday(date: &String) -> String {
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

pub fn round_decimal_pt(n: f64, point: u32) -> f64 {
    let val: f64 = 10i64.pow(point) as f64;
    (n * val).round() / val
}

pub fn get_directory_info() -> DirectoryInfo {
    let mut directory_info = DirectoryInfo {
        path_executed: String::new(),
        exchange_info_file_path: String::new(),
    };

    match env::current_exe() {
        Ok(exe_path) => {
            directory_info.path_executed = exe_path.to_str().unwrap().to_string();

            // truncate last binary file name
            let v: Vec<&str> = directory_info.path_executed.split('/').collect();
            let mut path_without_binary_name = String::new();
            for i in 0..v.len() - 1 {
                path_without_binary_name = path_without_binary_name + v[i] + "/";
            }

            // path_executed
            directory_info.path_executed = path_without_binary_name;

            // exchange_info_file_path
            directory_info.exchange_info_file_path =
                directory_info.path_executed.clone() + EXCHANGE_INFO_FILE_NAME;
        }
        Err(e) => {
            println!("failed to get current exe path: {e}");
            println!(
                "setupにて作成される「{}」はユーザールートフォルダに出力されます。",
                EXCHANGE_INFO_FILE_NAME
            );
            directory_info.path_executed = EXCHANGE_INFO_FILE_NAME.to_string();
            directory_info.exchange_info_file_path = EXCHANGE_INFO_FILE_NAME.to_string();
        }
    };
    directory_info
}

pub fn executed_shell_command(command: String) {
    std::process::Command::new(command).status().unwrap();
}
