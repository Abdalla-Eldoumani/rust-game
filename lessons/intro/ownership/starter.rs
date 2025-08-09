/// Fix `greet_name` so it borrows a string slice and does not take ownership.
/// Return a formatted greeting using the provided name.
///
/// CLI:
///   cargo run -- check --id intro/ownership
pub fn greet_name(_name: String) -> String {
    // TODO: make signature take &str, and format!("Hello, {name}!")
    _name
}


