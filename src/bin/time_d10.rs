use advent_calendar::days::{executer};
fn main() {
    let t = std::time::Instant::now();
    match executer(10) {
        Ok((p1, p2)) => println!("p1={p1}\np2={p2}\ntime={:.2?}", t.elapsed()),
        Err(e) => println!("err: {e}"),
    }
}
