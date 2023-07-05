use time::{Date, Time, UtcOffset};
use time::format_description::FormatItem;
use time::macros::format_description;

const DATE_FORMAT:&[FormatItem] = format_description!("[year]-[month]-[day]");
const TIME_FORMAT:&[FormatItem] = format_description!("[hour]:[minute]");
fn main() {
    let date = "2023-07-05";
    let time = "09:45";
    let offset_h = 2;
    let offset_m = 0;

    let unix = unix_from_datetime(date, time, offset_h, offset_m);

    println!("<t:{}>", unix);
    println!("<t:{}:R>", unix);
}

fn unix_from_datetime(date:&str, time:&str, offset_h:i8, offset_m:i8) -> i64{
    Date::parse(date, &DATE_FORMAT)
        .unwrap()
        .with_time(Time::parse(time, &TIME_FORMAT)
            .unwrap())
        .assume_offset(UtcOffset::from_hms(offset_h,offset_m,0)
            .unwrap())
        .unix_timestamp()
}