use crate::atomicassets::*;

// convert vector of any type to vector of strings
pub fn vec_to_string<T: ToString>(input: Vec<T>) -> Vec<String> {
    input.into_iter().map(|item| item.to_string()).collect()
}

// retrieve all schema names and dtypes into two lists
pub fn get_formats(event: &SchemasTableOperation) -> (Vec<String>, Vec<String>) {
    let mut names: Vec<String> = Vec::new();
    let mut dtypes: Vec<String> = Vec::new();
    for format in &event.format {
        names.push(format.name.to_string());
        dtypes.push(format.dtype.to_string());
    }
    (names, dtypes)
}