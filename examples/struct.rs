use bricke::bricke;
use jiff::{Timestamp as JiffTimestamp, civil::DateTime, tz::TimeZone};

// A dummy module to show that we can use a function from another module
mod utils {
    pub fn append_hello(input: String) -> String {
        format!("Hello, {}", input)
    }
}

// Convert a timestamp to a chrono DateTime
fn convert_ts_to_datetime(a: Timestamp) -> Result<DateTime, std::io::Error> {
    let ts = JiffTimestamp::new(a.seconds, 0)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?
        .to_zoned(TimeZone::UTC);

    Ok(ts.into())
}

struct Timestamp {
    seconds: i64,
}

struct Source {
    name: String,
    ts: Timestamp,
    hello: String,
}

#[derive(Debug)]
#[bricke(
    converter = "TryFrom",
    source = "Source",
    try_error_type = "std::io::Error"
)]
struct Target {
    #[allow(dead_code)]
    name: String,
    #[bricke_field(
        transform_fn = "convert_ts_to_datetime",
        rename = "ts",
        is_fallible = true
    )]
    #[allow(dead_code)]
    timestamp: DateTime,
    #[bricke_field(exclude = true)]
    #[allow(dead_code)]
    excluded: bool,
    #[bricke_field(transform_fn = "utils::append_hello")]
    hello: String,
}

fn main() {
    let b = Source {
        name: "Doudou".to_string(),
        ts: Timestamp {
            seconds: 1717708136,
        },
        hello: "doudou".to_string(),
    };

    let foo = Target::try_from(b);
    assert_eq!(foo.unwrap().hello, "Hello, doudou");
}
