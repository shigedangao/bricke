use bricke::bricke;

// A dummy module to show that we can use a function from another module
mod utils {
    pub fn append_hello(input: String) -> String {
        format!("Hello, {}", input)
    }
}

struct Source {
    hello: String,
}

#[derive(Debug)]
#[bricke(
    converter = "TryFrom",
    source = "Source",
    try_error_type = "std::io::Error"
)]
struct Target {
    #[bricke_field(transform_fn = "utils::append_hello")]
    hello: String,
}

fn main() {
    let b = Source {
        hello: "chaichai".to_string(),
    };

    let foo = Target::try_from(b);
    assert_eq!(foo.unwrap().hello, "Hello, chaichai");
}
