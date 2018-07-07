#![feature(rust_2018_preview, use_extern_macros)]
#![allow(unknown_lints)]
// TODO Add bare_trait_objects, but quick-error uses them for now.
#![warn(
    anonymous_parameters, missing_docs, trivial_numeric_casts, unused_import_braces,
    unused_qualifications, unreachable_pub
)]
// Necessary for some larger quick-error based errors.
#![recursion_limit = "128"]

//! livesplit-core is a library that provides a lot of functionality for creating a speedrun timer.
//!
//! # Examples
//!
//! ```
//! use livesplit_core::{Run, Segment, Timer, TimerPhase};
//!
//! // Create a run object that we can use with at least one segment.
//! let mut run = Run::new();
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Any%");
//! run.push_segment(Segment::new("Cap Kingdom"));
//!
//! // Create the timer from the run.
//! let mut timer = Timer::new(run).expect("Run with at least one segment provided");
//!
//! // Start a new attempt.
//! timer.start();
//! assert_eq!(timer.current_phase(), TimerPhase::Running);
//!
//! // Create a split.
//! timer.split();
//!
//! // The run should be finished now.
//! assert_eq!(timer.current_phase(), TimerPhase::Ended);
//!
//! // Reset the attempt and confirm that we want to store the attempt.
//! timer.reset(true);
//!
//! // The attempt is now over.
//! assert_eq!(timer.current_phase(), TimerPhase::NotRunning);
//! ```

// TODO reexport bug via use
pub extern crate livesplit_hotkey as hotkey;
pub use {indexmap, palette, parking_lot};

mod platform;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use crate::platform::*;

macro_rules! catch {
    ($($code:tt)*) => {
        (|| { Some({ $($code)* }) })()
    }
}

pub mod analysis;
pub mod comparison;
pub mod component;
mod hotkey_config;
mod hotkey_system;
mod image;
pub mod layout;
pub mod run;
pub mod settings;
#[cfg(test)]
mod tests_helper;
pub mod time;

pub use chrono::{DateTime, Utc};
pub use crate::{
    hotkey_config::HotkeyConfig, hotkey_system::HotkeySystem, image::Image,
    layout::{Component, Editor as LayoutEditor, GeneralSettings as GeneralLayoutSettings, Layout},
    run::{Attempt, Editor as RunEditor, Run, RunMetadata, Segment, SegmentHistory},
    time::{
        AtomicDateTime, GameTime, RealTime, SharedTimer, Time, TimeSpan, TimeStamp, Timer,
        TimerPhase, TimingMethod,
    },
};
