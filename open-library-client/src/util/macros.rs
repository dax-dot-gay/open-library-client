/// Combines a set of {key} = {value} pairs into an input for a query string.
/// Omits null values
#[macro_export]
macro_rules! params {
    ($($key:literal = $value:expr),*) => {
        &{
            let mut result: Vec<(String, String)> = Vec::new();
            $(if let Ok(ser) = serde_json::to_string(&$value) {
                if ser != "null" {
                    result.push((format!("{}", $key), ser.trim_matches('"').to_string()));
                }
            })*
            result
        }
    };
}

pub use params;
