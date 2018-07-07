//! The run module provides everything necessary for working with Runs, like
//! parsing and saving or editing them.
//!
//! # Examples
//!
//! ```
//! use livesplit_core::run::{Run, Segment};
//!
//! let mut run = Run::new();
//!
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Darker Side");
//!
//! run.push_segment(Segment::new("Cap Kingdom"));
//! run.push_segment(Segment::new("Cascade Kingdom"));
//! ```

mod attempt;
pub mod editor;
pub mod parser;
mod run;
mod run_metadata;
pub mod saver;
mod segment;
mod segment_history;

#[cfg(test)]
mod tests;

pub use self::{
    attempt::Attempt, editor::{Editor, RenameError}, run::{ComparisonError, ComparisonsIter, Run},
    run_metadata::RunMetadata, segment::Segment, segment_history::SegmentHistory,
};
