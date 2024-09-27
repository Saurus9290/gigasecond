use time::{Duration, OffsetDateTime};

pub fn after(start: OffsetDateTime) -> OffsetDateTime {
    start + Duration::seconds(100000000)
}

fn main() {
    let start = OffsetDateTime::now_utc();
    let result = after(start);
    println!("New time after 1 billion seconds: {}", result);
}
