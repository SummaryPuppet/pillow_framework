use futures::Future;
use std::pin::Pin;

pub struct FuturesHandler {
    func: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>,
}

impl FuturesHandler {
    pub async fn new(f: impl Future<Output = ()> + Send + Sync + 'static) -> FuturesHandler {
        FuturesHandler { func: Box::pin(f) }
    }

    pub async fn run(&mut self) {
        self.func.as_mut().await;
    }
}
