#![allow(missing_docs)]

crate use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

crate fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
