

use tokio::runtime;
use std::future::Future;
use pin_project::pin_project;

pub type AmbientExecutor = runtime::Handle;

pub fn try_get_ambient_executor()->Option<AmbientExecutor>{
    runtime::Handle::try_current().ok()
}

#[pin_project]
pub struct DetachedFuture<F:Future>(#[pin]F,runtime::Handle);

impl<F:Future> Future for DetachedFuture<F>{
    type Output = F::Output;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let tmp = self.project();
        let _d = tmp.1.enter();
        tmp.0.poll(cx)
    }
}