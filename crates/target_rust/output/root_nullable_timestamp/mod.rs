use chrono::{DateTime, FixedOffset};

pub type Root = Option<Box<DateTime<FixedOffset>>>;
