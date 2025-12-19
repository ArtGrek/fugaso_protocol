use std::{marker::PhantomData, time::{Duration, Instant}};
use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::channel::UnboundedSender;

pub trait Event {
    fn is_end(&self) -> bool;
}

#[async_trait]
pub trait Consumer {
    type E: Event;
    async fn accept(&mut self, e: Self::E);
}

#[derive(Debug)]
pub struct Holder<C: Consumer> {
    phantom: PhantomData<C>
}

impl<C: Consumer + Send + Sync + 'static> Holder<C> where C::E: Send {
    pub fn new(
        mut consumer: C,
        mut r: mpsc::UnboundedReceiver<C::E>
    ) -> Self {
        tokio::spawn(async move {
            while let Some(e) = r.recv().await {
                let is_end = e.is_end();
                consumer.accept(e).await;
                if is_end
                { break; }
            }
        });
        Self {
            phantom: PhantomData
        }
    }
}

#[derive(Debug)]
pub struct TimedHolder<C: Consumer> {
    access_time: Instant,
    sender: UnboundedSender<C::E>,
}

impl<C: Consumer + Send + Sync + 'static> TimedHolder<C> where C::E: Send {
    pub fn new(
        mut consumer: C,
    ) -> Self {
        let (s, mut r) = mpsc::unbounded_channel::<C::E>();
        tokio::spawn(async move {
            while let Some(e) = r.recv().await {
                let is_end = e.is_end();
                consumer.accept(e).await;
                if is_end
                { break; }
            }
        });
        Self {
            access_time: Instant::now(),
            sender: UnboundedSender::new(s),
        }
    }

    pub fn accept(&self, e: C::E) {
        self.sender.send(e, file!(), line!());
    }

    pub fn send(&self, e: C::E, file: &str, line: u32) {
        self.sender.send(e, file, line);
    }

    pub fn accept_on_dead(&self, duration: Duration, e: C::E) -> bool {
        let now = Instant::now();
        if now - self.access_time > duration {
            self.sender.send(e, file!(), line!());
            true
        } else { false }
    }
}
