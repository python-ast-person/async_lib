use std::future::{Future, IntoFuture};
//use async_executor::Task;

#[cfg(tokio_rt)]
#[path ="tokio_core"]
pub mod implementation;

//#[cfg(not(not_test))]
pub mod tokio_core; 

pub mod fs_api_threaded;
pub mod runtime_api_tokio;

//pub mod tokio_facade;

pub use fs_api_threaded as fs;
pub use runtime_api_tokio as runtime;

pub trait Runtime<'a>{
    type Task<T>:Future;

    fn spawn<T,>(&self,f:impl IntoFuture<Output=T>+'a)->Self::Task<T>;

    fn spawn_blocking<T>(&self,f:impl FnOnce()->T+Send+'a)->Self::Task<T>;

    fn detach<F:Future>(&self,f:F)->impl Future<Output = F::Output>;

    //fn scoped_spawn<'b,'scope,T>(&'b self,f:impl IntoFuture<Output = T>+'scope+'static)->impl Future<Output = T>+'b;
}

pub trait SendRuntime<'a>{
    type Task<T:Send>:Future+'a where T:'a;

    fn spawn<T:Send,F>(&self,f:F )->Self::Task<T> 
    where F:IntoFuture<Output=T>+'static,
        F::IntoFuture:Send,
        F::Output:Send;

    fn spawn_blocking<T:Send>(&self,f:impl FnOnce()->T+Send+'a)->Self::Task<T>;

    fn detach<F:Future>(&self,f:F)->impl Future<Output = F::Output>;

    //fn scoped_spawn<'b,'scope,T>(&'b self,f:impl IntoFuture<Output = T>+'scope+'static)->impl Future<Output = T>+'b;
}

impl<'a,U> Runtime<'a> for &'a U where U:Runtime<'a>+'a {
    type Task<T> = U::Task<T>;

    fn spawn<T>(&self,f:impl IntoFuture<Output=T>+'a)->U::Task<T> {
        (**self).spawn(f)
    }

    fn spawn_blocking<T>(&self,f:impl FnOnce()->T+Send+'a)->U::Task<T> {
        (**self).spawn_blocking(f)
    }

    fn detach<F:Future>(&self,f:F)->impl Future<Output = F::Output> {
        (**self).detach(f)
    }

    /*fn block_on<T>(&self,f:impl IntoFuture<Output = T>)->T {
        (**self).block_on(f)
    }*/

    /*fn scoped_spawn<'b,'scope,T>(&'b self,f:impl IntoFuture<Output = T>+'scope+'static)->impl Future<Output = T>+'b{
        (**self).scoped_spawn(f)
    }*/
}


fn main() {
    println!("Hello, world!");
}
