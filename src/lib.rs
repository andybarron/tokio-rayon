#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! Tokio's [`spawn_blocking`][spawn_blocking] and
//! [`block_in_place`][block_in_place] run blocking code on a potentially
//! large number of Tokio-controlled threads. This is suitable for blocking
//! I/O, but CPU-heavy operations are often better served by a fixed-size
//! thread pool. The Rayon crate provides exactly this, so... Why not
//! combine them? :)
//!
//! ```
//! # use tokio::sync::oneshot::error::RecvError;
//! # #[derive(Debug, PartialEq)]
//! # struct ExpensiveNft;
//! # fn do_some_crypto_stuff() -> ExpensiveNft { ExpensiveNft }
//! # tokio_test::block_on(async {
//! let nft = tokio_rayon::spawn_async(|| {
//!   do_some_crypto_stuff()
//! }).await?;
//!
//! assert_eq!(nft, ExpensiveNft);
//! # Ok::<(), RecvError>(())
//! # });
//! ```
//!
//! [spawn_blocking]: tokio::task::spawn_blocking
//! [block_in_place]: tokio::task::block_in_place

mod async_thread_pool;
mod global;
mod handle;

pub use async_thread_pool::*;
pub use global::*;
pub use handle::*;

/// Prelude module to bring in extension methods.
pub mod prelude {
    pub use super::async_thread_pool::*;
}

#[cfg(test)]
pub(crate) mod test {
    use rayon::ThreadPoolBuilder;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn init() {
        INIT.call_once(|| {
            ThreadPoolBuilder::new()
                .num_threads(1)
                .build_global()
                .unwrap();
        });
    }
}
