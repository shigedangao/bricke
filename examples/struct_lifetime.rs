use bricke::bricke;

fn transform_data<'a>(data: &'a [u8]) -> String {
    String::from_utf8_lossy(data).to_string()
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

#[bricke(converter = "From", source = "Input", lifetimes = 'a)]
struct OutputLifetime<'a> {
    data: &'a [u8],
}

fn main() {
    let input = Input { data: b"hello" };
    let output: Output = Output::from(input.clone());

    let output_lifetime = OutputLifetime::from(input);

    dbg!(output.data);
    dbg!(output_lifetime.data);
}
