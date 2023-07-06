use chrono::{prelude::Local, Days, FixedOffset, TimeZone, Utc};
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    loop {
        let now = Local::now();
        let year: i32 = now.format("%Y").to_string().parse().unwrap();
        let month: u32 = now.format("%m").to_string().parse().unwrap();
        let day: u32 = now.format("%d").to_string().parse().unwrap();
        let target = Utc.with_ymd_and_hms(year, month, day, 9, 50, 00).unwrap();
        let target = target.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
        if now > target {
            target.checked_add_days(Days::new(1)).unwrap();
        }
        let time = target.timestamp() - now.timestamp();
        sleep(Duration::from_secs(time.try_into().unwrap())).await;
        Command::new("open")
            .arg("/Applications/下班啦.app")
            .output()
            .expect("Failed to execute command");
    }
}
