use std::future::Future;

use anyhow::Result;
use tokio::sync::mpsc::{self, Sender};

use crate::{Page, Pages};

/// Runs a provider's listing with bounded backpressure, forwarding its first error and cancelling
/// its in-flight requests when the receiver closes. The producer sends successful pages only.
pub(crate) fn pages<T, F, R>(capacity: usize, load: F) -> Pages<T>
where
    T: Send + 'static,
    F: FnOnce(Sender<Result<Page<T>>>) -> R + Send + 'static,
    R: Future<Output = Result<()>> + Send,
{
    let (sink, pages) = mpsc::channel(capacity);
    tokio::spawn(async move {
        let result = tokio::select! {
            _ = sink.closed() => return,
            result = load(sink.clone()) => result,
        };
        if let Err(error) = result {
            sink.send(Err(error)).await.ok();
        }
    });
    pages
}
