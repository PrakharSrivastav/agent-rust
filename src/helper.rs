use tokio::time::{sleep, Duration};

pub async fn delay(ms: u64) {
    sleep(Duration::from_millis(ms)).await
}
