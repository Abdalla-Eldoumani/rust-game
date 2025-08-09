use tokio::sync::mpsc;

pub async fn fan_in(workers: usize) -> Vec<usize> {
    let (tx, mut rx) = mpsc::channel::<usize>(workers);
    for i in 0..workers {
        let txc = tx.clone();
        tokio::spawn(async move {
            let _ = txc.send(i).await;
        });
    }
    drop(tx);
    let mut out = Vec::new();
    while let Some(v) = rx.recv().await { out.push(v); }
    out
}


