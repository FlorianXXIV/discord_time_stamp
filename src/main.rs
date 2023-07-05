use time::{Date, Time, UtcOffset};
use time::format_description::FormatItem;
use time::macros::format_description;
use std::io;

const DATE_FORMAT:&[FormatItem] = format_description!("[year]-[month]-[day]");
const TIME_FORMAT:&[FormatItem] = format_description!("[hour]:[minute]");
const OFFSET_FORMAT:&[FormatItem] = format_description!("[offset_hour]:[offset_minute]");
fn main() {
    let mut buffer = String::new();

    println!("Input date as [year]-[month]-[day]");
    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let date = buffer.clone();
    buffer.clear();
    println!("Input time as [hour]:[minute]");
    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let time = buffer.clone();
    buffer.clear();
    println!("Input timezone (UTC offset) as +/-[hour]:[minute]");
    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let offset = buffer.clone();
    buffer.clear();

    let unix = unix_from_datetime(date.trim(), time.trim(), offset.trim());

    println!("<t:{}>", unix);
    println!("<t:{}:R>", unix);
}

fn unix_from_datetime(date:&str, time:&str, offset:&str) -> i64{
    Date::parse(date, &DATE_FORMAT)
        .unwrap()
        .with_time(Time::parse(time, &TIME_FORMAT)
            .unwrap())
        .assume_offset(UtcOffset::parse(offset, &OFFSET_FORMAT)
            .unwrap())
        .unix_timestamp()
}