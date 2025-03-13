use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

enum FetchJobState<T> {
    Fetching(Arc<Notify>),
    Ready(T),
}

#[derive(Clone)]
pub struct FetchJob<T> {
    state: Arc<Mutex<FetchJobState<T>>>,
}

impl<T: Clone> FetchJob<T> {
    pub fn new() -> FetchJob<T> {
        FetchJob {
            state: Arc::new(
                Mutex::new(
                    FetchJobState::Fetching(Arc::new(Notify::new()))
                )
            )
        }
    }

    pub async fn get(&self) -> T {
        let notify: Arc<Notify> = {
            let guard = self.state.lock().unwrap();
            match *guard {
                FetchJobState::Ready(ref t) => {
                    return t.clone();
                },
                FetchJobState::Fetching(ref n) => {
                    n.clone()
                },
            }
        };

        notify.notified().await;

        let guard = self.state.lock().unwrap();
        match *guard {
            FetchJobState::Ready(ref t) => {
                t.clone()
            },
            FetchJobState::Fetching(_) => {
                panic!("Unreachable in fetch_job get");
            },
        }
    }

    pub fn complete(&self, data: T) {
        let mut guard = self.state.lock().unwrap();
        match *guard {
            FetchJobState::Fetching(ref n) => {
                let notify = n.clone();
                *guard = FetchJobState::Ready(data);
                notify.notify_waiters();
            },
            _ => {
                *guard = FetchJobState::Ready(data);
            }
        }
    }
}
