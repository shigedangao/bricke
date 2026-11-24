use bricke::bricke;

enum Dataset {
    Name(String),
}

#[bricke(
    converter = "TryFrom",
    source = "Dataset",
    try_error_type = "std::io::Error"
)]
#[derive(PartialEq, Debug)]
enum Transformed {
    #[bricke_field(rename = "Name", transform_fn = "prettyfy_name", is_fallible = true)]
    PrettifyName(String),
}

fn prettyfy_name(name: String) -> Result<String, std::io::Error> {
    Ok(format!("Hello {name}"))
}

fn main() {
    let dataset = Dataset::Name("Rania !".to_string());
    let transformed = Transformed::try_from(dataset).unwrap();

    assert_eq!(
        transformed,
        Transformed::PrettifyName("Hello Rania !".to_string())
    );
}
