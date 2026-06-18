use bricke::bricke;

fn transform_data<'a>(data: &'a [u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}

fn transform_data_with_error<'a>(data: &'a [u8]) -> Result<&'a [u8], std::io::Error> {
    // todo do something with data
    Ok(data.iter().as_slice())
}

#[derive(Clone)]
struct Input<'a> {
    data: &'a [u8],
}

#[bricke(converter = "From", source = "Input", lifetimes = 'a)]
struct Output {
    #[bricke_field(transform_fn = "transform_data")]
    data: String,
}

#[bricke(converter = "TryFrom", source = "Input", try_error_type = "std::io::Error", lifetimes = 'a)]
struct OutputLifetime<'a> {
    #[bricke_field(transform_fn = "transform_data_with_error", is_fallible = true)]
    data: &'a [u8],
}

fn main() {
    let input = Input { data: b"hello" };
    let output: Output = Output::from(input.clone());

    let output_lifetime = OutputLifetime::try_from(input);

    dbg!(output.data);
    assert!(output_lifetime.is_ok());
}
