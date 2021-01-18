
use chrono::{DateTime, FixedOffset};

use serde::{Deserialize, Serialize};





pub type Root = Option<Box<DateTime<FixedOffset>>>;
