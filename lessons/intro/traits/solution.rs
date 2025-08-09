pub struct Person {
    pub name: String,
}

pub trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Person {
    fn describe(&self) -> String { format!("Person: {}", self.name) }
}


