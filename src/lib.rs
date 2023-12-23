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