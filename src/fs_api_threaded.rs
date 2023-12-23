
#![cfg_attr(nightly,feature(trace_macros) )]

//trace_macros!(true);

pub struct Reactor(pub(crate) tokio::runtime::Handle);

#[pin_project]
pub struct Task<'a,T:Send>(#[pin] tokio::task::JoinHandle<T>,PhantomData<&'a mut Reactor>);

impl<'a, T: Send> Future for Task<'a, T> {
    type Output = T;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.project().0.poll(cx).map(|z|z.ok().unwrap())
    }
}


use std::fs::{self,Metadata,ReadDir,Permissions};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::convert::AsRef;
use std::io::Result;

use futures::Future;
use pin_project::pin_project;

macro_rules! defer_to_std {
    ($(#[$($attrs:tt)*])* $(pub)? fn $fn_name:ident
        <$($gen_name:ident : $gen_path:path),*>
        (
            $($arg_name:ident:$arg_type:ty $(as $arg_convert:expr)?),*
        )->$ret_type:ty
        ) => {
            $(#[$($attrs)*])*
            pub fn $fn_name<$($gen_name : $gen_path),*>(&mut self, $($arg_name : $arg_type ),*)->Task<$ret_type>{
                $(let $arg_name = or_else_tokens!(($($arg_convert)?)($arg_name.as_ref().to_owned())));*;
                self.spawn_blocking(||fs::$fn_name($($arg_name),*))
            }
    };
    ()=>{};
}

macro_rules! or_else_tokens {
    (($($a:tt)+)($($b:tt)*)) => {
        $($a)*
    };
    (()($($b:tt)*))=>{
        $($b)*
    }
}

impl Reactor{
    fn spawn_blocking<T:Send + 'static>(&mut self,callback:impl FnOnce()->T+Send+'static)->Task<T>{
        Task(self.0.spawn_blocking(callback),PhantomData)
    }

    /*pub fn canonicalize < P : AsRef<Path> > (path : ident : P)-> Task <
           Result<PathBuf> >
           { let path = path.as_ref().to_owned(); fs :: canonicalize (path) }*/
    defer_to_std!(fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf>);
    defer_to_std!(fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64>);
    defer_to_std!(pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_std!(pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_std!(pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(
        original: P,
        link: Q
    ) -> Result<()>);
    defer_to_std!(pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata>);
    defer_to_std!(pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>>);
    defer_to_std!(pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>);
    defer_to_std!(pub fn read_link<P: AsRef<Path>>(path: P) -> Result<PathBuf>);
    defer_to_std!(pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>);
    defer_to_std!(pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_std!(pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_std!(pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()>);
    defer_to_std!(pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()>);
    defer_to_std!(
        #[allow(clippy::redundant_locals)]
        pub fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions as perm) -> Result<()>
    );
    defer_to_std!(pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata>);
    defer_to_std!(pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()>);
    defer_to_std!();
    defer_to_std!();
    defer_to_std!();
    defer_to_std!();
    defer_to_std!();
    defer_to_std!();
    defer_to_std!();
    defer_to_std!();
}