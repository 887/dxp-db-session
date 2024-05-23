#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(feature = "dbsession")]
mod db_session_storage;
#[cfg(feature = "dbsession")]
mod entities;

#[cfg(feature = "migration")]
pub mod migration;

#[cfg(feature = "dbsession")]
pub use db_session_storage::*;
