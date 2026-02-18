///! Example 'tracing' crate use.
/// See also: https://github.com/tugglecore/rust-tracing-primer
/// Levels: ERROR, WARN, INFO, DEBUG, TRACE
use tracing::{Level, debug, info, trace};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let sub = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(sub)
        .expect("Failed to set default subscriber");
    debug!("Start");

    let x = 10;
    let y = 15;
    info!("X {x}, Y {y}");

    let z = one(x);
    info!("Z {z}");

    let z = two(x, y);
    info!("Z {z}");

    let z = three(x, y, z);
    info!("Z {z}");

    debug!("Done");
}

fn one(a: i32) -> i32 {
    trace!("one( a {a} )");
    let value = a * a + 2 * a - 3;
    trace!("one(...) -> {value}");
    value
}

fn two(a: i32, b: i32) -> i32 {
    trace!("two( a {a} )");
    let value = 3 * b - 2 * a;
    trace!("two(...) -> {value}");
    value
}

fn three(a: i32, b: i32, c: i32) -> i32 {
    trace!("three( a {a}, b {b}, c {c} )");
    let value = a + b - c;
    trace!("three(...) -> {value}");
    value
}
