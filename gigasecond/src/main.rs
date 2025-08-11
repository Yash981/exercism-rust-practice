use time::{PrimitiveDateTime,OffsetDateTime};
fn main() {
    let now = OffsetDateTime::now_utc();
    let primitive = PrimitiveDateTime::new(now.date(), now.time());
    println!("{},{}",primitive,now.day())
}