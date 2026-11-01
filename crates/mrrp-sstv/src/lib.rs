//! WIP SSTV encoder and decoder
//!
//! # References
//!
//! - <http://lionel.cordesses.free.fr/gpages/sstv.html>
//! - <https://web.archive.org/web/20120505141047/http://www.cs.helsinki.fi/u/okraisan/slowrx/>
//! - <http://www.barberdsp.com/downloads/Dayton%20Paper.pdf>
//! - <https://web.archive.org/web/20120313215600/http://lionel.cordesses.free.fr/gpages/Cordesses.pdf>

pub mod color;
mod decoder;
mod encoder;
pub mod image;
pub mod modes;
pub mod state;

pub use decoder::{
    DecodeError,
    SstvDecoder,
};
pub use encoder::SstvEncoder;
use mrrp_core::signal::Frequency;

pub const LEADER_TONE: Frequency = 1900.0;
pub const LEADER_TIME: f64 = 0.300;

pub const LEADER_BREAK_TIME: f64 = 0.010;

pub const VIS_BIT_TIME: f64 = 0.030;
pub const VIS_LOW_TONE: Frequency = 1300.0;
pub const VIS_HIGH_TONE: Frequency = 1100.0;

// sync, leader break, vis start/stop
pub const SYNC_TONE: Frequency = 1200.0;

pub const PORCH_TONE: Frequency = 1500.0;

pub const CHANNEL_LOW_TONE: Frequency = 1500.0;
pub const CHANNEL_HIGH_TONE: Frequency = 2300.0;
