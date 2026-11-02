use bricke::bricke;
use jiff::{Timestamp as JiffTimestamp, civil::DateTime, tz::TimeZone};

// A dummy module to show that we can use a function from another module
mod utils {
    pub fn change_name(name: String) -> String {
        format!("Hello my name is {} nice to meet you all", name)
    }
}

// Convert a timestamp to a chrono DateTime
fn convert_ts_to_datetime(a: Timestamp) -> Result<DateTime, std::io::Error> {
    let ts = JiffTimestamp::new(a.seconds, 0)
        .map(|v| v.to_zoned(TimeZone::UTC))
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    Ok(ts.into())
}

struct Timestamp {
    seconds: i64,
}

struct Source {
    number: usize,
    ts: Timestamp,
    hello: String,
    #[allow(dead_code)]
    to_replace: Option<String>,
}

impl Source {
    fn add(number: usize) -> usize {
        number + number
    }
}

#[derive(Debug)]
#[bricke(
    converter = "TryFrom",
    source = "Source",
    try_error_type = "std::io::Error"
)]
struct Target {
    #[bricke_field(transform_fn = "Source::add")]
    number: usize,
    #[bricke_field(
        transform_fn = "convert_ts_to_datetime",
        rename = "ts",
        is_fallible = true
    )]
    #[allow(dead_code)]
    timestamp: DateTime,
    #[bricke_field(ignore)]
    #[allow(dead_code)]
    excluded: bool,
    #[bricke_field(transform_fn = "utils::change_name")]
    hello: String,
    #[bricke_field(default_value = "foo")]
    to_replace: Option<String>,
}

fn main() {
    let b = Source {
        number: 6,
        ts: Timestamp {
            seconds: 1717708136,
        },
        hello: "rara".to_string(),
        to_replace: None,
    };

    let foo = Target::try_from(b).unwrap();
    assert_eq!(foo.number, 12);
    assert_eq!(foo.to_replace.unwrap(), "foo");
    assert_eq!(foo.hello, "Hello my name is rara nice to meet you all");
}
