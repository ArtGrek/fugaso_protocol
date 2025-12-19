use log::error;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub struct UnboundedSender<T> {
    pub inner: mpsc::UnboundedSender<T>,
}

impl<T> UnboundedSender<T> {
    pub fn new(sender: mpsc::UnboundedSender<T>) -> Self {
        Self { inner: sender }
    }

    pub fn send_on(&self, message: T) -> Result<(), SendError<T>> {
        self.inner.send(message)
    }

    pub fn send(&self, message: T, file: &str, line: u32) {
        if let Err(_) = self.inner.send(message) {
            error!("error send on {} - {}", line, file)
        }
    }
}

impl<T> Clone for UnboundedSender<T>{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

#[derive(Debug)]
pub struct OneShotSender<T> {
    pub inner: oneshot::Sender<T>,
}

impl<T> OneShotSender<T> {
    pub fn new(sender: oneshot::Sender<T>) -> Self {
        Self { inner: sender }
    }

    pub fn send_on(self, message: T) -> Result<(), T> {
        self.inner.send(message)
    }

    pub fn send(self, message: T, file: &str, line: u32) {
        if let Err(_) = self.inner.send(message) {
            error!("error send on {} - {}", line, file)
        }
    }
}

#[macro_export]
macro_rules! one_shot {
    ($sender: expr, $msg: expr) => {
        if let Err(_) = $sender.send($msg) {
            error!("error send on {} - {}", line!(), file!())
        }
    };
}