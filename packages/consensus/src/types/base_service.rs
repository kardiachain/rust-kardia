use tokio::sync::Mutex;

#[derive(Debug)]
pub struct BaseService {
    running: Mutex<bool>,
}

impl BaseService {
    pub fn new() -> Self {
        Self {
            running: Mutex::new(false),
        }
    }

    pub async fn start(&self) {
        let mut running = self.running.lock().await;
        *running = true;
    }

    pub async fn stop(&self) {
        let mut running = self.running.lock().await;
        *running = false;
    }

    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }
}
