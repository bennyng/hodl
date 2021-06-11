#![warn(clippy::all, clippy::cargo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    code: String,
    name: String,
}
