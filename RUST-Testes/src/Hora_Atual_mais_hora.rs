use time::{OffsetDateTime, Duration};
use time::macros::offset;

fn main() {
   let  teste=5;
    let now = OffsetDateTime::now_utc().to_offset(offset!(-3)) + Duration::hours(teste);
    println!("Data e hora após a adição: {}", now);
}
