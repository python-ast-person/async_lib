
use core::future::IntoFuture;

use futures::Future;
use pin_project::pin_project;

pub struct Runtime(tokio::runtime::Handle);

pub type Task<T> = tokio::task::JoinHandle<T>;

#[pin_project]
pub struct DetachedFuture<F:Future>(#[pin]F,tokio::runtime::Handle);

impl Runtime{
    
    pub fn spawn<T:Send+'static,F>(&self,f:F )->Task<T> 
    where F:IntoFuture<Output=T>+'static,
        F::IntoFuture:Send,
        F::Output:Send{
        self.0.spawn(f.into_future())
    }

    pub fn spawn_blocking<T:Send+'static>(&self,f:impl FnOnce()->T+Send+'static)->Task<T> {
        self.0.spawn_blocking(f)
    }

    pub fn detach<F:Future>(&self,f:F)->impl Future<Output = F::Output> {
        DetachedFuture(f,self.0.clone())
    }

    pub fn get_reactor(&self)->crate::fs_api_threaded::Reactor{
        crate::fs_api_threaded::Reactor(self.0.clone())
    }
}

impl<F:Future> Future for DetachedFuture<F>{
    type Output = F::Output;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let tmp = self.project();
        let _d = tmp.1.enter();
        tmp.0.poll(cx)
    }
}

pub fn try_get_runtime()->Option<Runtime>{
    tokio::runtime::Handle::try_current().ok().map(Runtime)
}