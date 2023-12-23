use futures::Future;

pub fn spawn<F>(future: F) -> task::JoinHandle<F::Output> 
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
    {task::spawn(future)}


#[cfg(any(file_ops,not(not_test)))]
pub mod fs{

    use std::fs::{Metadata,ReadDir,Permissions};
    use std::path::{Path, PathBuf};
    use std::convert::AsRef;
    use std::io::Result;

    macro_rules! defer_to_runtime{
        ($(#[$($attrs:tt)*])* $(pub)? fn $fn_name:ident
        <$($gen_name:ident : $gen_path:path),*>
        (
            $($arg_name:ident:$arg_type:ty),*
        )->$ret_type:ty
        ) => {
            $(#[$($attrs)*])*
            pub async fn $fn_name<$($gen_name : $gen_path),*>($($arg_name : $arg_type ),*)->$ret_type{
                let mut reactor = crate::runtime_api_tokio::try_get_runtime().unwrap().get_reactor();
                reactor.$fn_name($($arg_name),*).await
            }
    };
    ()=>{};
    }

    defer_to_runtime!(pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata>);
    defer_to_runtime!(pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>>);
    defer_to_runtime!(pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>);
    defer_to_runtime!(pub fn read_link<P: AsRef<Path>>(path: P) -> Result<PathBuf>);
    defer_to_runtime!(pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>);
    defer_to_runtime!(pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_runtime!(pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_runtime!(pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_runtime!(pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()>);
    defer_to_runtime!(
        #[allow(clippy::redundant_locals)]
        pub fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> Result<()>
    );
    defer_to_runtime!(pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata>);
    defer_to_runtime!(pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()>);
}

#[cfg(any(runtime,not(not_test)))]
pub mod task{
    use std::{task::{Poll, Context}, pin::Pin};

    use futures::Future;
    use pin_project::pin_project;

    pub type JoinHandle<T> = crate::runtime::Task<T>;

    pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,{
            crate::runtime::try_get_runtime().unwrap().spawn(future)
        }

    pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R> 
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,{
            crate::runtime::try_get_runtime().unwrap().spawn_blocking(f)
        }

    #[pin_project]
    struct Unconstrained<T:Future>(#[pin] T);

    impl<T: Future>  Future for Unconstrained<T> {
        type Output = T::Output;

        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
            self.project().0.poll(cx)
        }
    }

    pub async fn yield_now() {
        /// Yield implementation
        struct YieldNow {
            yielded: bool,
        }
    
        impl Future for YieldNow {
            type Output = ();
    
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
    
                if self.yielded {
                    return Poll::Ready(());
                }
    
                self.yielded = true;
    
                cx.waker().wake_by_ref();
    
                Poll::Pending
            }
        }
    
        YieldNow { yielded: false }.await;
    }
    
}


#[cfg(any(io,not(not_test)))]
pub mod io{
    use futures_io as fio;

    pub use fio::{
        AsyncBufRead,
        AsyncRead,
        AsyncSeek,
        AsyncWrite,
    };
}