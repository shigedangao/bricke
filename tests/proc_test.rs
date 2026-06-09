use bricke::bricke;

struct Input {
    data: Vec<u8>,
}

fn transform_data(data: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(data)
}

#[bricke(
    converter = "TryFrom",
    source = "Input",
    try_error_type = "std::string::FromUtf8Error"
)]
struct Output {
    #[bricke_field(transform_fn = "transform_data", rename = "data", is_fallible = true)]
    transformed: String,
}

#[test]
fn expect_to_output_something() {
    let input = Input {
        data: b"Hello, World!".to_vec(),
    };

    let output = Output::try_from(input);
    assert!(output.is_ok());
    assert_eq!(output.unwrap().transformed, "Hello, World!");
}
