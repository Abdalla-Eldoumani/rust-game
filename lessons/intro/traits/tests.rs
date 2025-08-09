use crate::{Person, Describable};

#[test]
fn person_describe() {
    let p = Person { name: "Ferris".into() };
    let got = p.describe();
    assert_eq!(got, "Person: Ferris");
}


