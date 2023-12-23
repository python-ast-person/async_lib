

use tokio::runtime;
use std::future::{Future, IntoFuture};
use crate::*;
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

impl SendRuntime<'static> for runtime::Handle{
    type Task<T:Send+'static> = tokio::task::JoinHandle<T>;

    fn spawn<T:Send+'static,F>(&self,f:F )->Self::Task<T> 
    where F:IntoFuture<Output=T>+'static,
        F::IntoFuture:Send,
        F::Output:Send{
        self.spawn(f.into_future())
    }

    fn spawn_blocking<T:Send+'static>(&self,f:impl FnOnce()->T+Send+'static)->Self::Task<T> {
        self.spawn_blocking(f)
    }

    fn detach<F:Future>(&self,f:F)->impl Future<Output = F::Output> {
        DetachedFuture(f,self.clone())
    }
}