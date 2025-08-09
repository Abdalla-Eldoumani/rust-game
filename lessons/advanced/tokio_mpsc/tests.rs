#[tokio::test]
async fn collects_from_tasks() {
    let out = exercise_sandbox::fan_in(6).await;
    let mut got = out.clone();
    got.sort();
    assert_eq!(got, vec![0,1,2,3,4,5]);
}


