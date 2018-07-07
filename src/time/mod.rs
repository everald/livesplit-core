//! The timer module provides everything necessary for working with times and
//! measuring them.

mod atomic_date_time;
pub mod formatter;
mod time;
mod time_span;
mod time_stamp;
mod timer;
mod timer_phase;
mod timing_method;

pub use self::{
    atomic_date_time::AtomicDateTime, time::{GameTime, RealTime, Time},
    time_span::{ParseError, TimeSpan}, time_stamp::TimeStamp,
    timer::{CreationError as TimerCreationError, SharedTimer, Timer}, timer_phase::TimerPhase,
    timing_method::TimingMethod,
};
