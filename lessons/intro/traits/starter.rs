pub struct Person {
    pub name: String,
}

// Define a trait `Describable` with `describe(&self) -> String`
pub trait Describable {
    fn describe(&self) -> String;
}

// Implement it for `Person` returning format!("Person: {}", name)


