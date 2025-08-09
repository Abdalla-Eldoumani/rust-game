use exercise_sandbox::Msg::*;

#[test]
fn handles_variants() {
    assert_eq!(exercise_sandbox::handle(Ping).unwrap(), 0);
    assert_eq!(exercise_sandbox::handle(Data(3)).unwrap(), 3);
    assert!(exercise_sandbox::handle(Data(-1)).is_err());
    assert!(exercise_sandbox::handle(Quit).is_err());
}


