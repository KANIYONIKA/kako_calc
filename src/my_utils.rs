use chrono::{Duration, TimeZone, Utc};

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
