//! Abstract port interfaces and mock adapters for Soroban Sentinel.

pub mod events;
pub mod mock;
pub mod reader;

pub use events::EventStreamReader;
pub use mock::MockLedgerReader;
pub use reader::{LatestLedgerInfo, LedgerReader, RawLedgerEntry, ReaderHealth};
