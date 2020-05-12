use snafu::Snafu;

// Likely candidates to clash with generated code
mod core {}
mod std {}
mod snafu {}

#[derive(Debug, Snafu)]
enum VariantNamedNone {
    None,
}

#[derive(Debug, Snafu)]
enum VariantNamedSome<T> {
    Some { value: T },
}

#[derive(Debug, Snafu)]
enum VariantNamedOk<T> {
    Ok { value: T },
}

#[derive(Debug, Snafu)]
enum VariantNamedErr<T> {
    Err { value: T },
}
