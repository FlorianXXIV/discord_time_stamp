use time::{Date, Time, UtcOffset};
use time::format_description::FormatItem;
use time::macros::format_description;

const DATE_FORMAT:&[FormatItem] = format_description!("[year]-[month]-[day]");
const TIME_FORMAT:&[FormatItem] = format_description!("[hour]:[minute]");
const OFFSET_FORMAT:&[FormatItem] = format_description!("[offset_hour]:[offset_minute]");

pub struct TimeStamp;

impl TimeStamp{
    fn unix_from_datetime( date:&str, time:&str, offset:&str) -> Result<i64,String>{
        let mut c = offset.chars();
        let offset = match c.next(){
            Some('A'..='Z') => match offset {
                "Y" | "AoE" => Ok("-12:00"),
                "X" | "NUT" | "SST" => Ok("-11:00"),
                "W" | "CKT" | "HST" | "TAHT" => Ok("-10:00"),
                "MART" => Ok("-09:30"),
                "V" | "AKST" | "GAMT" | "HDT" => Ok("-09:00"),
                "U" | "PST" | "AKDT" => Ok("-08:00"),
                "T" | "PDT" | "MST" => Ok("-07:00"),
                "S" | "CST" | "EAST" | "GALT" | "MDT" => Ok("-06:00"),
                "R" | "EST" | "CDT" | "ACT" | "COT" | "EASST" | "ECT" | "PET" => Ok("-05:00"),
                "Q" | "EDT" | "AST" | "BOT" | "CIDST" | "CLT" | "GYT" | "PYT" | "VET" => Ok("-04:00"),
                "P" | "ADT" | "AMST" | "ART" | "BRT" | "CLST" | "FKST" | "GFT" | "PMST" | "PYST" | "ROTT" | "SRT" | "UYT" | "WARST" | "WGT" => Ok("-03:00"),
                "NDT" => Ok("-02:30"),
                "O" | "BRST" | "FNT" | "PMDT" | "UYST" | "WGST" => Ok("-02:00"),
                "N" | "AZOT" | "CVT" | "EGT" => Ok("-01:00"),
                "Z" | "GMT" | "WET" | "AZOST" | "EGST" | "WT" => Ok("00:00"),
                "A" | "BST" | "CET" | "WEST" | "WAT" | "WST" => Ok("01:00"),
                "B" | "CEST" | "CAT" | "EET" | "SAST" => Ok("02:00"),
                "C" | "EEST" | "MSK" | "EAT" | "IDT" => Ok("03:00"),
                "IRST" => Ok("03:30"),
                "D" | "GST" | "AMT" | "AZT" | "GET" | "MUT" | "RET" | "SAMT" | "SCT" => Ok("04:00"),
                "IRDT" | "AFT" => Ok("04:30"),
                "E" | "YEKT" | "AQTT" | "AZST" | "MAWT" | "MVT" | "ORAT" | "TFT" | "TJT" | "TMT" | "UZT" => Ok("05:00"),
                "IST" => Ok("05:30"),
                "NPT" => Ok("05:45"),
                "F" | "ALMT" | "BTT" | "IOT" | "KGT" | "OMST" | "QYZT" | "VOST" => Ok("06:00"),
                "MMT" | "CCT" => Ok("06:30"),
                "G" | "CXT" | "DAVT" | "HOVT" | "ICT" | "KRAT" | "NOVT" | "WIB" => Ok("07:00"),
                "H" | "AWST" | "BNT" | "CAST" | "CHOT" | "HKT" | "HOVST" | "IRKT" | "MYT" | "PHT" | "SGT" | "ULAT" | "WITA" => Ok("08:00"),
                "ACWST" => Ok("08:45"),
                "I" | "CHOST" | "JST" | "KST" | "PWT" | "TLT" | "ULAST" | "WIT" | "YAKT" => Ok("09:00"),
                "ACST" => Ok("09:30"),
                "K" | "AEST" | "CHUT" | "ChST" | "DDUT" | "PGT" | "VLAT" | "YAPT" => Ok("10:00"),
                "ACDT" | "LHST" => Ok("10:30"),
                "L" | "AEDT" | "KOST" | "LHDT" | "MAGT" | "NCT" | "NFT" | "PONT" | "SAKT" | "SBT" | "SRET" | "VUT" => Ok("11:00"),
                "M" | "ANAT" | "FJT" | "GILT" | "MHT" | "NFDT" | "NRT" | "NZST" | "PETT" | "TVT" | "WAKT" | "WFT" => Ok("12:00"),
                "CHAST" => Ok("12:45"),
                "TOT" => Ok("13:00"),
                "CHADT" => Ok("13:45"),
                "TOST" => Ok("14:00"),
                _ => Err("Timezone Not included")
            },
            Some('+') | Some('-') | Some('0'..='1') => Ok(offset),
            _ => Err("Wrong Format")
        };
        Ok(Date::parse(date, &DATE_FORMAT)
                        .unwrap()
                        .with_time(Time::parse(time, &TIME_FORMAT)
                        .unwrap())
                        .assume_offset(UtcOffset::parse(offset.expect("Failed to get offset"),
                                                        &OFFSET_FORMAT)
                        .unwrap())
                        .unix_timestamp())
    }

    pub fn get_discord_time_stamp(date:&str, time:&str, offset:&str) -> String {
        "<t:".to_owned() +
            TimeStamp::unix_from_datetime(date, time, offset)
                .expect("could not parse datetime")
                .to_string()
                .as_str()
            + ">"
    }
    pub fn get_rel_time_stamp(date:&str, time:&str, offset:&str) -> String {
        "<t:".to_owned() +
            TimeStamp::unix_from_datetime(date, time, offset)
                .expect("could not parse datetime")
                .to_string()
                .as_str()
            + ":R>"
    }
    pub fn get_dynamic_time_stamp(date:&str,time:&str,offset:&str,formatter:&str) -> String{
        "<t:".to_owned() +
            TimeStamp::unix_from_datetime(date,time,offset)
                .expect("could not parse datetime")
                .to_string()
                .as_str()
            + ":"
            + formatter
            + ">"
    }
    pub fn get_actual_time_stamp(date:&str, time:&str, offset:&str) -> String {
        "t:".to_owned() +
            TimeStamp::unix_from_datetime(date, time, offset)
                .expect("could not parse datetime")
                .to_string()
                .as_str()
    }
    pub fn get_rel_actual_time_stamp(date:&str, time:&str, offset:&str) -> String {
        "t:".to_owned() +
            TimeStamp::unix_from_datetime(date, time, offset)
                .expect("could not parse datetime")
                .to_string()
                .as_str()
            + ":R"
    }
    pub fn get_dynamic_actual_time_stamp(date:&str,time:&str,offset:&str,formatter:&str) -> String{
        "t:".to_owned() +
            TimeStamp::unix_from_datetime(date,time,offset)
                .expect("could not parse datetime")
                .to_string()
                .as_str()
            + ":"
            + formatter
    }
}


