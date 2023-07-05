use time::{Date, Time, UtcOffset};
use time::format_description::FormatItem;
use time::macros::format_description;

const DATE_FORMAT:&[FormatItem] = format_description!("[year]-[month]-[day]");
const TIME_FORMAT:&[FormatItem] = format_description!("[hour]:[minute]");
const OFFSET_FORMAT:&[FormatItem] = format_description!("[offset_hour]:[offset_minute]");

pub struct TimeStamp;

impl TimeStamp{
    fn unix_from_datetime(date:&str, time:&str, offset:&str) -> i64{
        Date::parse(date, &DATE_FORMAT)
            .unwrap()
            .with_time(Time::parse(time, &TIME_FORMAT)
                .unwrap())
            .assume_offset(UtcOffset::parse(offset, &OFFSET_FORMAT)
                .unwrap())
            .unix_timestamp()
    }

    pub fn get_discord_time_stamp(date:&str, time:&str, offset:&str) -> String {
        "<t:".to_owned()+ &*TimeStamp::unix_from_datetime(date, time, offset).to_string() +">"
    }
}


