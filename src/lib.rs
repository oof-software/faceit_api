#![feature(int_roundings)]
#![allow(dead_code)]

mod client;
pub use client::Client;
mod mapping;
pub use mapping::{MapStats, Mapping};
mod matches;
pub use matches::{Match, Matches};
mod nickname;
mod rate_limit;
pub use rate_limit::{rate_limit, RateLimitIter};
mod room;
pub use room::Room;
mod search;
pub use search::Search;
mod shared;
mod stats;
pub use stats::Stats;
mod player_info;
mod types;
